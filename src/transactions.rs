//! Transaction operations for Redis Rhai integration

use crate::client::RedisClient;
use crate::generic::redis_value_to_dynamic;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn multi(&mut self) -> bool {
        let mut conn = self.conn.lock().unwrap();
        redis::cmd("MULTI")
            .query::<String>(&mut *conn)
            .map(|_| true)
            .unwrap_or(false)
    }

    pub fn exec(&mut self) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match redis::cmd("EXEC").query::<redis::Value>(&mut *conn) {
            Ok(redis::Value::Array(values)) => {
                values.into_iter().map(redis_value_to_dynamic).collect()
            }
            _ => vec![],
        }
    }

    pub fn discard(&mut self) -> bool {
        let mut conn = self.conn.lock().unwrap();
        redis::cmd("DISCARD")
            .query::<String>(&mut *conn)
            .map(|_| true)
            .unwrap_or(false)
    }
}

pub fn register_transaction_methods(engine: &mut Engine) {
    engine
        .register_fn("multi", RedisClient::multi)
        .register_fn("exec", RedisClient::exec)
        .register_fn("discard", RedisClient::discard);
}
