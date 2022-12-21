use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
use kafka_common::message_type::{GetMessageType, MessageType};
use log::*;
use serde::Serialize;
use std::sync::atomic::Ordering::{self, Relaxed};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::runtime::Runtime;

use crate::geyser_neon_config::GeyserPluginKafkaConfig;
use crate::kafka_producer::KafkaProducer;
use crate::kafka_producer_stats::{ContextWithStats, Stats};

async fn serialize_and_send<T: Serialize + GetMessageType>(
    config: Arc<GeyserPluginKafkaConfig>,
    mut producer: KafkaProducer,
    message: T,
    hash: String,
    stats: Arc<Stats>,
) {
    let message_type = message.get_type();
    let (topic, counter_send_success, counter_send_failed) = match message_type {
        MessageType::UpdateAccount => (
            &config.update_account_topic,
            &stats.kafka_update_account,
            &stats.kafka_error_update_account,
        ),
        MessageType::UpdateSlot => (
            &config.update_slot_topic,
            &stats.kafka_update_slot,
            &stats.kafka_error_update_slot,
        ),
        MessageType::NotifyTransaction => (
            &config.notify_transaction_topic,
            &stats.kafka_notify_transaction,
            &stats.kafka_error_notify_transaction,
        ),
        MessageType::NotifyBlock => (
            &config.notify_block_topic,
            &stats.kafka_notify_block,
            &stats.kafka_error_notify_block,
        ),
    };

    match serde_json::to_string(&message) {
        Ok(message) => {
            if let Err(e) = producer.send(topic, &message, &hash, None).await {
                counter_send_failed.inc();
                error!(
                    "Producer cannot send {message_type} message with size {}, error: {}",
                    message.len(),
                    e.0
                );
                return;
            }
            counter_send_success.inc();
            stats
                .kafka_bytes_tx
                .inner()
                .fetch_add(message.len() as u64, Ordering::Relaxed);
        }
        Err(e) => {
            stats.kafka_error_serialize.inc();
            error!("Failed to serialize {message_type} message, error {e}");
        }
    }
}

pub async fn update_account_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateAccount>,
    ctx_stats: ContextWithStats,
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone(), ctx_stats);
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for update_account_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_account) = rx.recv_async().await {
                let producer = producer.clone();
                let stats = producer.get_stats();
                let config = config.clone();
                let hash = update_account.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(config, producer, update_account, hash, stats).await;
                });
            }
        }
    } else {
        panic!(
            "Failed to create Kafka producer for update_account_loop: {:?}",
            producer_result.err()
        );
    }
}

pub async fn update_slot_status_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateSlotStatus>,
    ctx_stats: ContextWithStats,
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone(), ctx_stats);
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for update_slot_status_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_slot_status) = rx.recv_async().await {
                let producer = producer.clone();
                let stats = producer.get_stats();
                let config = config.clone();
                let hash = update_slot_status.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(config, producer, update_slot_status, hash, stats).await;
                });
            }
        }
    } else {
        panic!(
            "Failed to create Kafka producer for update_slot_status_loop: {:?}",
            producer_result.err()
        );
    }
}

pub async fn notify_transaction_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyTransaction>,
    ctx_stats: ContextWithStats,
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone(), ctx_stats);
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for notify_transaction_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_transaction) = rx.recv_async().await {
                let producer = producer.clone();
                let stats = producer.get_stats();
                let config = config.clone();
                let hash = notify_transaction.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(config, producer, notify_transaction, hash, stats).await;
                });
            }
        }
    } else {
        panic!(
            "Failed to create Kafka producer for notify_transaction_loop: {:?}",
            producer_result.err()
        );
    }
}

pub async fn notify_block_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<NotifyBlockMetaData>,
    ctx_stats: ContextWithStats,
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone(), ctx_stats);
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for notify_block_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_block) = rx.recv_async().await {
                let producer = producer.clone();
                let stats = producer.get_stats();
                let config = config.clone();
                let hash = notify_block.get_hash().to_string();

                runtime.spawn(async move {
                    serialize_and_send(config, producer, notify_block, hash, stats).await;
                });
            }
        }
    } else {
        panic!(
            "Failed to create Kafka producer for notify_transaction_loop: {:?}",
            producer_result.err()
        );
    }
}
