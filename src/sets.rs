//! Set operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn sadd(&mut self, key: &str, member: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.sadd::<_, _, i64>(key, member).unwrap_or(0)
    }

    pub fn srem(&mut self, key: &str, member: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.srem::<_, _, i64>(key, member).unwrap_or(0)
    }

    pub fn sismember(&mut self, key: &str, member: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.sismember::<_, _, bool>(key, member).unwrap_or(false)
    }

    pub fn smembers(&mut self, key: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.smembers::<_, Vec<String>>(key) {
            Ok(members) => members.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn scard(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.scard::<_, i64>(key).unwrap_or(0)
    }
}

pub fn register_set_methods(engine: &mut Engine) {
    engine
        .register_fn("sadd", RedisClient::sadd)
        .register_fn("srem", RedisClient::srem)
        .register_fn("sismember", RedisClient::sismember)
        .register_fn("smembers", RedisClient::smembers)
        .register_fn("scard", RedisClient::scard);
}
