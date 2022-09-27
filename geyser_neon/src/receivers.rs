use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
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
        while !should_stop.load(Relaxed) {
            if let Ok(update_account) = rx.recv_async().await {
                let message = serde_json::to_string(&update_account)
                    .expect("Failed to serialize UpdateAccount message");
                let _ = producer
                    .send(&config.update_account_topic, &message, "", None)
                    .await;
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
        while !should_stop.load(Relaxed) {
            if let Ok(update_slot_status) = rx.recv_async().await {
                let message = serde_json::to_string(&update_slot_status)
                    .expect("Failed to serialize UpdateSlotStatus message");
                let _ = producer
                    .send(&config.update_slot_topic, &message, "", None)
                    .await;
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
        while !should_stop.load(Relaxed) {
            if let Ok(notify_transaction) = rx.recv_async().await {
                let message = serde_json::to_string(&notify_transaction)
                    .expect("Failed to serialize NotifyTransaction message");
                let _ = producer
                    .send(&config.notify_transaction_topic, &message, "", None)
                    .await;
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
        while !should_stop.load(Relaxed) {
            if let Ok(notify_block) = rx.recv_async().await {
                let message = serde_json::to_string(&notify_block)
                    .expect("Failed to serialize NotifyBlockMetaData message");
                let _ = producer
                    .send(&config.notify_block_topic, &message, "", None)
                    .await;
            }
        }
    }
}
