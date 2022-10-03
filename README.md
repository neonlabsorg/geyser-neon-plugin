### Geyser neon

The geyser_neon asynchronously sends data from the [geyser plugins interface](https://docs.solana.com/developing/plugins/geyser-plugins) to Kafka using [rdkafka](https://github.com/fede1024/rust-rdkafka).

### Requirements
1. Solana Validator 1.11.10
2. Kafka cluster

### Configuration File Format
The plugin is configured using the input configuration file. An example
configuration file looks like the following:
```
{
    "libpath": "/home/user/libgeyser_neon.so",
    "brokers_list": "167.235.75.213:9092,159.69.197.26:9092,167.235.151.85:9092",
    "update_account_topic": "update_account",
    "update_slot_topic": "update_slot",
    "notify_transaction_topic": "notify_transaction",
    "notify_block_topic": "notify_block",
    "kafka_logging_format": "DEBUG",
    "message_timeout": "5000"
}
```
In order to load the plugin at the start of the Solana validator it is necessary to add the parameter
**--geyser-plugin-config** with the path to the config above.
