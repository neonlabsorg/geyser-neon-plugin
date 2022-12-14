use rdkafka::{ClientContext, Statistics};

pub struct ContextWithStats;

impl ClientContext for ContextWithStats {
    fn stats(&self, _stats: Statistics) {}
}
