use std::time::Duration;

use rdkafka::{
    error::KafkaResult,
    message::OwnedHeaders,
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig,
};

pub struct KafkaProducer<'a> {
    pub future_producer: FutureProducer,
    pub brokers_list: &'a str,
    pub topic: &'a str,
    pub logging_format: &'a str,
}

impl<'a> KafkaProducer<'a> {
    pub fn new(
        brokers_list: &'a str,
        topic_name: &'a str,
        message_timeout: &'a str,
        logging_format: &'a str,
    ) -> KafkaResult<Self> {
        let future_producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers_list)
            .set("message.timeout.ms", message_timeout)
            .set("message.send.max.retries", i32::MAX.to_string())
            .create()?;

        Ok(KafkaProducer {
            future_producer,
            brokers_list,
            topic: topic_name,
            logging_format,
        })
    }

    pub async fn send(
        &mut self,
        message: &str,
        key: &str,
        headers: Option<OwnedHeaders>,
    ) -> OwnedDeliveryResult {
        let mut future_record = FutureRecord::to(self.topic).payload(message).key(key);

        future_record.headers = headers;

        self.future_producer
            .send(future_record, Duration::from_secs(0))
            .await
    }
}
