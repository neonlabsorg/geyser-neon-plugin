use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeyserPluginKafkaConfig {
    pub brokers_list: String,
    pub update_account_topic: String,
    pub update_slot_topic: String,
    pub notify_transaction_topic: String,
    pub notify_block_topic: String,
    pub kafka_logging_format: String,
    pub message_timeout: String,
}
