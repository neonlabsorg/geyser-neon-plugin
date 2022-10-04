use crate::kafka_structs::{
    NotifyBlockMetaData, NotifyTransaction, UpdateAccount, UpdateSlotStatus,
};

impl UpdateAccount {
    pub fn get_hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        match &self.account {
            crate::kafka_structs::KafkaReplicaAccountInfoVersions::V0_0_1(account_info) => {
                hasher.update(account_info.pubkey.as_slice());
                hasher.update(&account_info.write_version.to_le_bytes());
            }
            crate::kafka_structs::KafkaReplicaAccountInfoVersions::V0_0_2(account_info) => {
                hasher.update(account_info.pubkey.as_slice());
                hasher.update(&account_info.write_version.to_le_bytes());
            }
        }

        hasher.update(&self.slot.to_le_bytes());
        hasher.finalize().to_string()
    }
}

impl UpdateSlotStatus {
    pub fn get_hash(&self) -> String {
        self.slot.to_string() + &self.status.to_string()
    }
}

impl NotifyTransaction {
    pub fn get_hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();

        match &self.transaction_info {
            crate::kafka_structs::KafkaReplicaTransactionInfoVersions::V0_0_1(transaction_info) => {
                hasher.update(transaction_info.signature.as_ref());
            }
            crate::kafka_structs::KafkaReplicaTransactionInfoVersions::V0_0_2(transaction_info) => {
                hasher.update(transaction_info.signature.as_ref());
            }
        }

        hasher.update(&self.slot.to_le_bytes());
        hasher.finalize().to_string()
    }
}

impl NotifyBlockMetaData {
    pub fn get_hash(&self) -> String {
        match &self.block_info {
            crate::kafka_structs::KafkaReplicaBlockInfoVersions::V0_0_1(block_info) => {
                block_info.blockhash.to_string()
            }
        }
    }
}
