use prometheus_client::metrics::counter::Counter;
use rdkafka::{ClientContext, Statistics};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Default)]
pub struct Stats {
    pub kafka_sent_update_account: Counter<u64, AtomicU64>,
    pub kafka_sent_update_slot: Counter<u64, AtomicU64>,
    pub kafka_sent_notify_transaction: Counter<u64, AtomicU64>,
    pub kafka_sent_notify_block: Counter<u64, AtomicU64>,
    pub kafka_error_update_account: Counter<u64, AtomicU64>,
    pub kafka_error_update_slot: Counter<u64, AtomicU64>,
    pub kafka_error_notify_transaction: Counter<u64, AtomicU64>,
    pub kafka_error_notify_block: Counter<u64, AtomicU64>,
}

#[derive(Default, Clone)]
pub struct ContextWithStats {
    pub stats: Arc<Stats>,
}

impl ClientContext for ContextWithStats {
    fn stats(&self, _stats: Statistics) {}
}
