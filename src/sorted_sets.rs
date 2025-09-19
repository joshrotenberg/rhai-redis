//! Sorted set operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn zadd(&mut self, key: &str, score: f64, member: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.zadd::<_, _, _, i64>(key, member, score).unwrap_or(0)
    }

    pub fn zrem(&mut self, key: &str, member: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.zrem::<_, _, i64>(key, member).unwrap_or(0)
    }

    pub fn zcard(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.zcard::<_, i64>(key).unwrap_or(0)
    }

    pub fn zscore(&mut self, key: &str, member: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.zscore::<_, _, Option<f64>>(key, member) {
            Ok(Some(score)) => Dynamic::from(score),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    pub fn zrange(&mut self, key: &str, start: i64, stop: i64) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.zrange::<_, Vec<String>>(key, start as isize, stop as isize) {
            Ok(members) => members.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }
}

pub fn register_sorted_set_methods(engine: &mut Engine) {
    engine
        .register_fn("zadd", RedisClient::zadd)
        .register_fn("zrem", RedisClient::zrem)
        .register_fn("zcard", RedisClient::zcard)
        .register_fn("zscore", RedisClient::zscore)
        .register_fn("zrange", RedisClient::zrange);
}
