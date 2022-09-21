use rdkafka::{error::KafkaResult, producer::FutureProducer, ClientConfig};

#[allow(dead_code)]
pub struct KafkaProducer {
    future_producer: FutureProducer,
    brokers_list: String,
    topic: String,
    message_timeout: String,
    logging_format: String,
}

impl KafkaProducer {
    pub fn new(
        brokers_list: String,
        topic_name: String,
        message_timeout: String,
        logging_format: String,
    ) -> KafkaResult<Self> {
        let future_producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers_list)
            .set("message.timeout.ms", &message_timeout)
            .create()?;

        Ok(KafkaProducer {
            future_producer,
            brokers_list,
            topic: topic_name,
            message_timeout,
            logging_format,
        })
    }
}
