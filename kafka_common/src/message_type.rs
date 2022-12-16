use std::fmt;

use crate::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};

pub enum MessageType {
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

pub trait GetMessageType {
    fn get_type(&self) -> MessageType;
}

impl GetMessageType for NotifyBlockMetaData {
    fn get_type(&self) -> MessageType {
        MessageType::NotifyBlock
    }
}

impl GetMessageType for NotifyTransaction {
    fn get_type(&self) -> MessageType {
        MessageType::NotifyTransaction
    }
}

impl GetMessageType for UpdateAccount {
    fn get_type(&self) -> MessageType {
        MessageType::UpdateAccount
    }
}

impl GetMessageType for UpdateSlotStatus {
    fn get_type(&self) -> MessageType {
        MessageType::UpdateSlot
    }
}
