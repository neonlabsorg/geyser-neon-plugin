use std::sync::Arc;

use rdkafka::{
    error::KafkaResult,
    message::OwnedHeaders,
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    util::Timeout,
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
            .set("security.protocol", &config.security_protocol)
            .set("sasl.mechanism", &config.sasl_mechanism)
            .set("sasl.username", &config.sasl_username)
            .set("sasl.password", &config.sasl_password)
            .set_log_level((&config.kafka_log_level).into())
            .set(
                "message.send.max.retries",
                &config.producer_send_max_retries,
            )
            .set(
                "queue.buffering.max.messages",
                &config.producer_queue_max_messages,
            )
            .set("message.max.bytes", &config.producer_message_max_bytes)
            .set("request.timeout.ms", &config.producer_request_timeout_ms)
            .set("retry.backoff.ms", &config.producer_retry_backoff_ms)
            .set("compression.codec", &config.compression_codec)
            .set("compression.level", &config.compression_level)
            .set("batch.size", &config.batch_size)
            .set("batch.num.messages", &config.batch_num_messages)
            .set("linger.ms", &config.linger_ms)
            .set("acks", &config.acks)
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
            .send(future_record, Timeout::Never)
            .await
    }
}
