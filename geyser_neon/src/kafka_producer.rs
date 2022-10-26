use std::{sync::Arc, time::Duration};

use rdkafka::{
    error::KafkaResult,
    message::OwnedHeaders,
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::geyser_neon_config::GeyserPluginKafkaConfig;

#[derive(Clone)]
pub struct KafkaProducer {
    pub future_producer: FutureProducer,
    pub config: Arc<GeyserPluginKafkaConfig>,
}

impl KafkaProducer {
    pub fn new(config: Arc<GeyserPluginKafkaConfig>) -> KafkaResult<Self> {
        let future_producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers_list)
            .set("message.timeout.ms", &config.message_timeout_ms)
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanism", "PLAIN")
            .set("sasl.username", &config.sasl_user)
            .set("sasl.password", &config.sasl_password)
            .set_log_level((&config.kafka_log_level).into())
            .set(
                "message.send.max.retries",
                &config.producer_send_max_retries,
            )
            .create()?;

        Ok(KafkaProducer {
            future_producer,
            config,
        })
    }

    pub async fn send(
        &mut self,
        topic: &str,
        message: &str,
        key: &str,
        headers: Option<OwnedHeaders>,
    ) -> OwnedDeliveryResult {
        let mut future_record = FutureRecord::to(topic).payload(message).key(key);

        future_record.headers = headers;

        self.future_producer
            .send(future_record, Duration::from_secs(0))
            .await
    }
}
