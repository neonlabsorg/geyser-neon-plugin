### Configuration File Format

The plugin is configured using the input configuration file. An example
configuration file looks like the following:

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
