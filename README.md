## Geyser neon
The geyser_neon asynchronously sends data from the [geyser plugins interface](https://docs.solana.com/developing/plugins/geyser-plugins) to Kafka using [rdkafka](https://github.com/fede1024/rust-rdkafka).

### Requirements
1. Solana Validator 1.14.10
2. Kafka cluster
3. Geyser plugin must be compiled with the same version of Rust as the validator itself

### Configuration File Format
The plugin is configured using the input configuration file, read the [librdkafka documentation](https://docs.confluent.io/5.5.1/clients/librdkafka/md_CONFIGURATION.html) to set the optimal producer parameters.
\
In order to configure an SSL certificate, see the [librdkafka documentation](https://github.com/edenhill/librdkafka/blob/master/INTRODUCTION.md#ssl).
\
Path to the log file is **/var/log/neon/geyser.log**
\
An example configuration file looks like the following:
```
{
    "libpath": "/home/user/libgeyser_neon.so",
    "brokers_list": "167.235.75.213:9092,159.69.197.26:9092,167.235.151.85:9092",
    "sasl_username": "username",
    "sasl_password": "password",
    "sasl_mechanism": "SCRAM-SHA-512",
    "security_protocol": "SASL_SSL",
    "update_account_topic": "update_account",
    "update_slot_topic": "update_slot",
    "notify_transaction_topic": "notify_transaction",
    "notify_block_topic": "notify_block",
    "producer_send_max_retries": "100",
    "producer_queue_max_messages": "125000",
    "producer_message_max_bytes": "104857600",
    "producer_request_timeout_ms": "100000",
    "producer_retry_backoff_ms": "1000",
    "internal_queue_capacity": "30000",
    "compression_codec": "lz4",
    "compression_level": "12",
    "batch_size": "104857600",
    "batch_num_messages": "10000",
    "linger_ms": "20",
    "acks": "-1",
    "message_timeout_ms": "100000",
    "kafka_log_level": "Info",
    "global_log_level": "Info"
}
```
In order to load the plugin at the start of the Solana validator it is necessary to add the parameter
**--geyser-plugin-config** with the path to the config above.
\
\
To configure the logging level for librdkafka you should use:
```
RUST_LOG="librdkafka=trace,rdkafka::client=debug"
```
This will configure the logging level of librdkafka to trace, and the level of the client module of the Rust client to debug.

