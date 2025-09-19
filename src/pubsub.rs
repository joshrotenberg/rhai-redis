//! Pub/Sub operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::Engine;

impl RedisClient {
    pub fn publish(&mut self, channel: &str, message: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.publish::<_, _, i64>(channel, message).unwrap_or(0)
    }
}

pub fn register_pubsub_methods(engine: &mut Engine) {
    engine.register_fn("publish", RedisClient::publish);
}
