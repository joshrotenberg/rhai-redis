//! Key operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn expire(&mut self, key: &str, seconds: i64) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.expire::<_, bool>(key, seconds).unwrap_or(false)
    }

    pub fn ttl(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.ttl::<_, i64>(key).unwrap_or(-2)
    }

    pub fn keys(&mut self, pattern: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.keys::<_, Vec<String>>(pattern) {
            Ok(keys) => keys.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn dbsize(&mut self) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        redis::cmd("DBSIZE").query::<i64>(&mut *conn).unwrap_or(0)
    }

    pub fn flushdb(&mut self) -> bool {
        let mut conn = self.conn.lock().unwrap();
        redis::cmd("FLUSHDB")
            .query::<String>(&mut *conn)
            .map(|_| true)
            .unwrap_or(false)
    }
}

pub fn register_key_methods(engine: &mut Engine) {
    engine
        .register_fn("expire", RedisClient::expire)
        .register_fn("ttl", RedisClient::ttl)
        .register_fn("keys", RedisClient::keys)
        .register_fn("dbsize", RedisClient::dbsize)
        .register_fn("flushdb", RedisClient::flushdb);
}
