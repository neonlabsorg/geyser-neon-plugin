use std::{
    fs::File,
    io::Read,
    sync::{atomic::AtomicBool, Arc},
};

use chrono::Utc;
use flume::Sender;
use kafka_common::kafka_structs::{
    KafkaReplicaAccountInfoVersions, KafkaReplicaBlockInfoVersions,
    KafkaReplicaTransactionInfoVersions, KafkaSlotStatus, NotifyBlockMetaData, NotifyTransaction,
    UpdateAccount, UpdateSlotStatus,
};
use rdkafka::config::RDKafkaLogLevel;
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError;
use thiserror::Error;
use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

/// Main entry for the Kafka plugin
use {
    log::*,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaTransactionInfoVersions, Result, SlotStatus,
    },
};

use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
    Config, Logger,
};

use flume::Receiver;

use crate::{
    build_info::get_build_info,
    geyser_neon_config::GeyserPluginKafkaConfig,
    kafka_producer_stats::ContextWithStats,
    prometheus::start_prometheus,
    receivers::{
        notify_block_loop, notify_transaction_loop, update_account_loop, update_slot_status_loop,
    },
};

pub struct GeyserPluginKafka {
    runtime: Arc<Runtime>,
    config: Option<Arc<GeyserPluginKafkaConfig>>,
    logger: &'static Logger,
    account_tx: Option<Sender<UpdateAccount>>,
    slot_status_tx: Option<Sender<UpdateSlotStatus>>,
    transaction_tx: Option<Sender<NotifyTransaction>>,
    block_metadata_tx: Option<Sender<NotifyBlockMetaData>>,
    should_stop: Arc<AtomicBool>,
    prometheus_jhandle: Option<JoinHandle<()>>,
    update_account_jhandle: Option<JoinHandle<()>>,
    update_slot_status_jhandle: Option<JoinHandle<()>>,
    notify_transaction_jhandle: Option<JoinHandle<()>>,
    notify_block_jhandle: Option<JoinHandle<()>>,
}

impl Default for GeyserPluginKafka {
    fn default() -> Self {
        Self::new()
    }
}

impl GeyserPluginKafka {
    pub fn new() -> Self {
        let runtime = Arc::new(
            runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to initialize Tokio runtime"),
        );

        let logger: &'static Logger = fast_log::init(Config::new().console().file_split(
            "/var/log/neon/geyser.log",
            LogSize::KB(512),
            RollingType::All,
            LogPacker {},
        ))
        .expect("Failed to initialize fast_log");

        let should_stop = Arc::new(AtomicBool::new(false));

        Self {
            runtime,
            config: None,
            logger,
            account_tx: None,
            slot_status_tx: None,
            transaction_tx: None,
            block_metadata_tx: None,
            should_stop,
            update_account_jhandle: None,
            update_slot_status_jhandle: None,
            notify_transaction_jhandle: None,
            notify_block_jhandle: None,
            prometheus_jhandle: None,
        }
    }

    fn run(
        &mut self,
        config: Arc<GeyserPluginKafkaConfig>,
        account_rx: Receiver<UpdateAccount>,
        slot_status_rx: Receiver<UpdateSlotStatus>,
        transaction_rx: Receiver<NotifyTransaction>,
        block_metadata_rx: Receiver<NotifyBlockMetaData>,
        should_stop: Arc<AtomicBool>,
    ) {
        info!(
            "Rdkafka logging level will be set to {:?}",
            Into::<RDKafkaLogLevel>::into(&config.kafka_log_level)
        );

        self.logger.set_level((&config.global_log_level).into());

        info!(
            "Global logging level is set to {:?}",
            Into::<LevelFilter>::into(&config.global_log_level)
        );

        info!("{}", get_build_info());

        let ctx_stats = ContextWithStats::default();

        let prometheus_port = config
            .prometheus_port
            .parse()
            .unwrap_or_else(|e| panic!("Wrong prometheus port number, error: {e}"));

        let prometheus_jhandle = Some(
            self.runtime
                .spawn(start_prometheus(ctx_stats.stats.clone(), prometheus_port)),
        );

        let update_account_jhandle = Some(self.runtime.spawn(update_account_loop(
            self.runtime.clone(),
            config.clone(),
            account_rx,
            ctx_stats.clone(),
            should_stop.clone(),
        )));

        let update_slot_status_jhandle = Some(self.runtime.spawn(update_slot_status_loop(
            self.runtime.clone(),
            config.clone(),
            slot_status_rx,
            ctx_stats.clone(),
            should_stop.clone(),
        )));

        let notify_transaction_jhandle = Some(self.runtime.spawn(notify_transaction_loop(
            self.runtime.clone(),
            config.clone(),
            transaction_rx,
            ctx_stats.clone(),
            should_stop.clone(),
        )));

        let notify_block_jhandle = Some(self.runtime.spawn(notify_block_loop(
            self.runtime.clone(),
            config,
            block_metadata_rx,
            ctx_stats,
            should_stop,
        )));

        self.prometheus_jhandle = prometheus_jhandle;
        self.update_account_jhandle = update_account_jhandle;
        self.update_slot_status_jhandle = update_slot_status_jhandle;
        self.notify_transaction_jhandle = notify_transaction_jhandle;
        self.notify_block_jhandle = notify_block_jhandle;
    }
}

#[derive(Error, Debug)]
pub enum GeyserPluginKafkaError {}

impl std::fmt::Debug for GeyserPluginKafka {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl GeyserPlugin for GeyserPluginKafka {
    fn name(&self) -> &'static str {
        "GeyserPluginKafka"
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let result: serde_json::Result<GeyserPluginKafkaConfig> = serde_json::from_str(&contents);
        match result {
            Err(err) => {
                return Err(GeyserPluginError::ConfigFileReadError {
                    msg: format!(
                        "The config file is not in the JSON format expected: {:?}",
                        err
                    ),
                })
            }
            Ok(config) => {
                let config = Arc::new(config);
                self.config = Some(config.clone());
                let internal_queue_capacity = config
                    .internal_queue_capacity
                    .parse::<usize>()
                    .unwrap_or(30000);

                let (account_tx, account_rx) = flume::bounded(internal_queue_capacity);
                let (slot_status_tx, slot_status_rx) = flume::bounded(internal_queue_capacity);
                let (transaction_tx, transaction_rx) = flume::bounded(internal_queue_capacity);
                let (block_metadata_tx, block_metadata_rx) =
                    flume::bounded(internal_queue_capacity);

                self.account_tx = Some(account_tx);
                self.slot_status_tx = Some(slot_status_tx);
                self.transaction_tx = Some(transaction_tx);
                self.block_metadata_tx = Some(block_metadata_tx);

                self.run(
                    config,
                    account_rx,
                    slot_status_rx,
                    transaction_rx,
                    block_metadata_rx,
                    self.should_stop.clone(),
                );
            }
        }

        Ok(())
    }

    fn on_unload(&mut self) {
        self.should_stop
            .store(true, std::sync::atomic::Ordering::SeqCst);
        info!("Unloading plugin: {}", self.name());
        let update_account_jhandle = self.update_account_jhandle.take();
        let update_slot_status_jhandle = self.update_slot_status_jhandle.take();
        let notify_transaction_jhandle = self.notify_transaction_jhandle.take();
        let notify_block_jhandle = self.notify_block_jhandle.take();

        self.runtime.block_on(async move {
            if let Some(handle) = update_account_jhandle {
                let _ = handle.await;
            }

            if let Some(handle) = update_slot_status_jhandle {
                let _ = handle.await;
            }

            if let Some(handle) = notify_transaction_jhandle {
                let _ = handle.await;
            }

            if let Some(handle) = notify_block_jhandle {
                let _ = handle.await;
            }
        });

        self.logger.flush();
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> Result<()> {
        let account: KafkaReplicaAccountInfoVersions = account.into();
        let account_tx = self.account_tx.clone();

        let update_account = UpdateAccount {
            account,
            slot,
            is_startup,
        };

        match account_tx
            .expect("Channel was not created!")
            .send(update_account)
        {
            Ok(_) => (),
            Err(e) => error!("Failed to send UpdateAccount, error: {e}"),
        }
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> Result<()> {
        let status: KafkaSlotStatus = status.into();
        let slot_status_tx = self.slot_status_tx.clone();
        let retrieved_time = Utc::now().naive_utc();

        let update_account = UpdateSlotStatus {
            slot,
            parent,
            status,
            retrieved_time,
        };

        match slot_status_tx
            .expect("Channel was not created!")
            .send(update_account)
        {
            Ok(_) => (),
            Err(e) => error!("Failed to send UpdateSlotStatus, error: {e}"),
        }

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> Result<()> {
        info!("Notifying the end of startup for accounts notifications");

        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction_info: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> Result<()> {
        let transaction_info: KafkaReplicaTransactionInfoVersions = transaction_info.into();
        let transaction_tx = self.transaction_tx.clone();

        let notify_transaction = NotifyTransaction {
            transaction_info,
            slot,
        };

        match transaction_tx
            .expect("Channel was not created!")
            .send(notify_transaction)
        {
            Ok(_) => (),
            Err(e) => error!("Failed to send NotifyTransaction, error: {e}"),
        }

        Ok(())
    }

    fn notify_block_metadata(&mut self, block_info: ReplicaBlockInfoVersions) -> Result<()> {
        let block_info: KafkaReplicaBlockInfoVersions = block_info.into();
        let block_metadata_tx = self.block_metadata_tx.clone();

        let notify_block = NotifyBlockMetaData { block_info };

        match block_metadata_tx
            .expect("Channel was not created!")
            .send(notify_block)
        {
            Ok(_) => (),
            Err(e) => error!("Failed to send NotifyBlockMetaData, error: {e}"),
        }

        Ok(())
    }

    /// Check if the plugin is interested in account data
    /// Default is true -- if the plugin is not interested in
    /// account data, please return false.
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in transaction data
    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginKafka pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserPluginKafka::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
