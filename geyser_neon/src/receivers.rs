use flume::Receiver;
use kafka_common::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicBool, Arc};

pub async fn update_account_loop(rx: Receiver<UpdateAccount>, should_stop: Arc<AtomicBool>) {
    while !should_stop.load(Relaxed) {
        if let Ok(_update_account) = rx.recv_async().await {}
    }
}

pub async fn update_slot_status_loop(rx: Receiver<UpdateSlotStatus>, should_stop: Arc<AtomicBool>) {
    while !should_stop.load(Relaxed) {
        if let Ok(_update_slot_status) = rx.recv_async().await {}
    }
}

pub async fn notify_transaction_loop(
    rx: Receiver<NotifyTransaction>,
    should_stop: Arc<AtomicBool>,
) {
    while !should_stop.load(Relaxed) {
        if let Ok(_notify_transaction) = rx.recv_async().await {}
    }
}

pub async fn notify_block_loop(rx: Receiver<NotifyBlockMetaData>, should_stop: Arc<AtomicBool>) {
    while !should_stop.load(Relaxed) {
        if let Ok(_notify_block) = rx.recv_async().await {}
    }
}
