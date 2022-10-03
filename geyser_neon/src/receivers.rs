use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
use log::*;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicBool, Arc};

use crate::geyser_neon_config::GeyserPluginKafkaConfig;
use crate::kafka_producer::KafkaProducer;

pub async fn update_account_loop(
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateAccount>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(mut producer) = KafkaProducer::new(
        &config.brokers_list,
        &config.update_account_topic,
        &config.message_timeout,
        &config.kafka_logging_format,
    ) {
        info!("Created KafkaProducer for update_account_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_account) = rx.recv_async().await {
                match serde_json::to_string(&update_account) {
                    Ok(message) => {
                        if let Err(e) = producer.send(&message, "", None).await {
                            error!("Producer cannot send UpdateAccount message, error: {:?}", e);
                        }
                    }
                    Err(e) => error!("Failed to serialize UpdateAccount message, error {e}"),
                }
            }
        }
    }
}

pub async fn update_slot_status_loop(
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateSlotStatus>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(mut producer) = KafkaProducer::new(
        &config.brokers_list,
        &config.update_slot_topic,
        &config.message_timeout,
        &config.kafka_logging_format,
    ) {
        info!("Created KafkaProducer for update_slot_status_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_slot_status) = rx.recv_async().await {
                match serde_json::to_string(&update_slot_status) {
                    Ok(message) => {
                        if let Err(e) = producer.send(&message, "", None).await {
                            error!(
                                "Producer cannot send UpdateSlotStatus message, error: {:?}",
                                e
                            );
                        }
                    }
                    Err(e) => error!("Failed to serialize UpdateSlotStatus message, error {e}"),
                }
            }
        }
    }
}

pub async fn notify_transaction_loop(
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyTransaction>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(mut producer) = KafkaProducer::new(
        &config.brokers_list,
        &config.notify_transaction_topic,
        &config.message_timeout,
        &config.kafka_logging_format,
    ) {
        info!("Created KafkaProducer for notify_transaction_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_transaction) = rx.recv_async().await {
                match serde_json::to_string(&notify_transaction) {
                    Ok(message) => {
                        if let Err(e) = producer.send(&message, "", None).await {
                            error!(
                                "Producer cannot send NotifyTransaction message, error: {:?}",
                                e
                            );
                        }
                    }
                    Err(e) => error!("Failed to serialize NotifyTransaction message, error {e}"),
                }
            }
        }
    }
}

pub async fn notify_block_loop(
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyBlockMetaData>,
    should_stop: Arc<AtomicBool>,
) {
    if let Ok(mut producer) = KafkaProducer::new(
        &config.brokers_list,
        &config.notify_block_topic,
        &config.message_timeout,
        &config.kafka_logging_format,
    ) {
        info!("Created KafkaProducer for notify_block_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_block) = rx.recv_async().await {
                match serde_json::to_string(&notify_block) {
                    Ok(message) => {
                        if let Err(e) = producer.send(&message, "", None).await {
                            error!(
                                "Producer cannot send NotifyBlockMetaData message, error: {:?}",
                                e
                            );
                        }
                    }
                    Err(e) => error!("Failed to serialize NotifyBlockMetaData message, error {e}"),
                }
            }
        }
    }
}
