//! String operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn get(&mut self, key: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.get::<_, Option<String>>(key) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(e) => {
                eprintln!("Redis error: {}", e);
                Dynamic::UNIT
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.set::<_, _, ()>(key, value).is_ok()
    }

    pub fn del(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.del::<_, i64>(key).unwrap_or(0)
    }

    pub fn exists(&mut self, key: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.exists::<_, bool>(key).unwrap_or(false)
    }

    pub fn incr(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.incr::<_, _, i64>(key, 1).unwrap_or(0)
    }

    pub fn incrby(&mut self, key: &str, increment: i64) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.incr::<_, _, i64>(key, increment).unwrap_or(0)
    }

    pub fn decr(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.decr::<_, _, i64>(key, 1).unwrap_or(0)
    }

    pub fn decrby(&mut self, key: &str, decrement: i64) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.decr::<_, _, i64>(key, decrement).unwrap_or(0)
    }
}

pub fn register_string_methods(engine: &mut Engine) {
    engine
        .register_fn("get", RedisClient::get)
        .register_fn("set", RedisClient::set)
        .register_fn("del", RedisClient::del)
        .register_fn("exists", RedisClient::exists)
        .register_fn("incr", RedisClient::incr)
        .register_fn("incrby", RedisClient::incrby)
        .register_fn("decr", RedisClient::decr)
        .register_fn("decrby", RedisClient::decrby);
}
