use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeyserPluginKafkaConfig {
    // Servers list in kafka format
    pub brokers_list: String,
    pub update_account_topic: String,
    pub update_slot_topic: String,
    pub notify_transaction_topic: String,
    pub notify_block_topic: String,
    // From 0 to 2147483647 (i32::MAX),
    pub producer_send_max_retries: String,
    // This value is only enforced locally and limits the time a produced message waits for successful delivery.
    // A time of 0 is infinite.
    // This is the maximum time librdkafka may use to deliver a message (including retries)
    // From 0 to 2147483647 (i32::MAX)
    pub message_timeout_ms: String,
}
