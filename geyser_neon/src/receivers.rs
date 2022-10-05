use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
use log::*;
use serde::Serialize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::runtime::Runtime;

use crate::geyser_neon_config::GeyserPluginKafkaConfig;
use crate::kafka_producer::KafkaProducer;

pub async fn serialize_and_send<T: Serialize>(
    config: Arc<GeyserPluginKafkaConfig>,
    mut producer: KafkaProducer,
    message: T,
    hash: String,
) {
    match serde_json::to_string(&message) {
        Ok(message) => {
            if let Err(e) = producer
                .send(&config.update_account_topic, &message, &hash, None)
                .await
            {
                error!("Producer cannot send message, error: {:?}", e);
            }
        }
        Err(e) => error!("Failed to serialize message, error {e}"),
    }
}

pub async fn update_account_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateAccount>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(producer) = KafkaProducer::new(config.clone()) {
        info!("Created KafkaProducer for update_account_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_account) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                runtime.spawn(async move {
                    let hash = update_account.get_hash();
                    serialize_and_send(config, producer, update_account, hash).await;
                });
            }
        }
    }
}

pub async fn update_slot_status_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateSlotStatus>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(producer) = KafkaProducer::new(config.clone()) {
        info!("Created KafkaProducer for update_slot_status_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_slot_status) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                runtime.spawn(async move {
                    let hash = update_slot_status.get_hash();
                    serialize_and_send(config, producer, update_slot_status, hash).await;
                });
            }
        }
    }
}

pub async fn notify_transaction_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyTransaction>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(producer) = KafkaProducer::new(config.clone()) {
        info!("Created KafkaProducer for notify_transaction_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_transaction) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                runtime.spawn(async move {
                    let hash = notify_transaction.get_hash();
                    serialize_and_send(config, producer, notify_transaction, hash).await;
                });
            }
        }
    }
}

pub async fn notify_block_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyBlockMetaData>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(producer) = KafkaProducer::new(config.clone()) {
        info!("Created KafkaProducer for notify_block_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_block) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                runtime.spawn(async move {
                    let hash = notify_block.get_hash().to_string();
                    serialize_and_send(config, producer, notify_block, hash).await;
                });
            }
        }
    }
}
