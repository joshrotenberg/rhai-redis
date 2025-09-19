//! List operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn lpush(&mut self, key: &str, value: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.lpush::<_, _, i64>(key, value).unwrap_or(0)
    }

    pub fn rpush(&mut self, key: &str, value: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.rpush::<_, _, i64>(key, value).unwrap_or(0)
    }

    pub fn lpop(&mut self, key: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.lpop::<_, Option<String>>(key, None) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    pub fn rpop(&mut self, key: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.rpop::<_, Option<String>>(key, None) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    pub fn llen(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.llen::<_, i64>(key).unwrap_or(0)
    }

    pub fn lrange(&mut self, key: &str, start: i64, stop: i64) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.lrange::<_, Vec<String>>(key, start as isize, stop as isize) {
            Ok(vals) => vals.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn lindex(&mut self, key: &str, index: i64) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.lindex::<_, Option<String>>(key, index as isize) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    pub fn lset(&mut self, key: &str, index: i64, value: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.lset::<_, _, ()>(key, index as isize, value).is_ok()
    }
}

pub fn register_list_methods(engine: &mut Engine) {
    engine
        .register_fn("lpush", RedisClient::lpush)
        .register_fn("rpush", RedisClient::rpush)
        .register_fn("lpop", RedisClient::lpop)
        .register_fn("rpop", RedisClient::rpop)
        .register_fn("llen", RedisClient::llen)
        .register_fn("lrange", RedisClient::lrange)
        .register_fn("lindex", RedisClient::lindex)
        .register_fn("lset", RedisClient::lset);
}
