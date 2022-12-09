use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
use log::*;
use serde::Serialize;
use std::fmt;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::runtime::Runtime;

use crate::geyser_neon_config::GeyserPluginKafkaConfig;
use crate::kafka_producer::KafkaProducer;

enum MessageType {
    UpdateAccount,
    UpdateSlot,
    NotifyTransaction,
    NotifyBlock,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::UpdateAccount => write!(f, "UpdateAccount"),
            MessageType::UpdateSlot => write!(f, "UpdateSlot"),
            MessageType::NotifyTransaction => write!(f, "NotifyTransaction"),
            MessageType::NotifyBlock => write!(f, "NotifyBlock"),
        }
    }
}

async fn serialize_and_send<T: Serialize>(
    config: Arc<GeyserPluginKafkaConfig>,
    mut producer: KafkaProducer,
    message: T,
    message_type: MessageType,
    hash: String,
) {
    let topic = match message_type {
        MessageType::UpdateAccount => &config.update_account_topic,
        MessageType::UpdateSlot => &config.update_slot_topic,
        MessageType::NotifyTransaction => &config.notify_transaction_topic,
        MessageType::NotifyBlock => &config.notify_block_topic,
    };

    match serde_json::to_string(&message) {
        Ok(message) => {
            if let Err(e) = producer.send(topic, &message, &hash, None).await {
                error!(
                    "Producer cannot send {message_type} message with size {}, error: {}",
                    message.len(),
                    e.0
                );
                trace!(
                    "Producer cannot send {message_type} message, error: {}, serialized message {message}",
                    e.0
                );
            }
        }
        Err(e) => error!("Failed to serialize {message_type} message, error {e}"),
    }
}

pub async fn update_account_loop(
    runtime: Arc<Runtime>,
    config: Arc<GeyserPluginKafkaConfig>,
    rx: Receiver<UpdateAccount>,
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone());
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for update_account_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_account) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                let hash = update_account.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(
                        config,
                        producer,
                        update_account,
                        MessageType::UpdateAccount,
                        hash,
                    )
                    .await;
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
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone());
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for update_slot_status_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(update_slot_status) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                let hash = update_slot_status.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(
                        config,
                        producer,
                        update_slot_status,
                        MessageType::UpdateSlot,
                        hash,
                    )
                    .await;
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
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone());
    if let Ok(producer) = KafkaProducer::new(config.clone()) {
        info!("Created KafkaProducer for notify_transaction_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_transaction) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                let hash = notify_transaction.get_hash();

                runtime.spawn(async move {
                    serialize_and_send(
                        config,
                        producer,
                        notify_transaction,
                        MessageType::NotifyTransaction,
                        hash,
                    )
                    .await;
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
    should_stop: Arc<AtomicBool>,
) {
    let producer_result = KafkaProducer::new(config.clone());
    if let Ok(producer) = producer_result {
        info!("Created KafkaProducer for notify_block_loop!");
        while !should_stop.load(Relaxed) {
            if let Ok(notify_block) = rx.recv_async().await {
                let producer = producer.clone();
                let config = config.clone();
                let hash = notify_block.get_hash().to_string();

                runtime.spawn(async move {
                    serialize_and_send(
                        config,
                        producer,
                        notify_block,
                        MessageType::NotifyBlock,
                        hash,
                    )
                    .await;
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
