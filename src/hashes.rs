//! Hash operations for Redis Rhai integration

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hset::<_, _, _, i64>(key, field, value).unwrap_or(0)
    }

    pub fn hget(&mut self, key: &str, field: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.hget::<_, _, Option<String>>(key, field) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    pub fn hdel(&mut self, key: &str, field: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel::<_, _, i64>(key, field).unwrap_or(0)
    }

    pub fn hexists(&mut self, key: &str, field: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.hexists::<_, _, bool>(key, field).unwrap_or(false)
    }

    pub fn hlen(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hlen::<_, i64>(key).unwrap_or(0)
    }

    pub fn hkeys(&mut self, key: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.hkeys::<_, Vec<String>>(key) {
            Ok(keys) => keys.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn hvals(&mut self, key: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        match conn.hvals::<_, Vec<String>>(key) {
            Ok(vals) => vals.into_iter().map(Dynamic::from).collect(),
            Err(_) => vec![],
        }
    }

    pub fn hgetall(&mut self, key: &str) -> rhai::Map {
        let mut conn = self.conn.lock().unwrap();
        let mut map = rhai::Map::new();

        if let Ok(values) = redis::cmd("HGETALL")
            .arg(key)
            .query::<Vec<String>>(&mut *conn)
        {
            for chunk in values.chunks(2) {
                if chunk.len() == 2 {
                    map.insert(chunk[0].clone().into(), Dynamic::from(chunk[1].clone()));
                }
            }
        }

        map
    }
}

pub fn register_hash_methods(engine: &mut Engine) {
    engine
        .register_fn("hset", RedisClient::hset)
        .register_fn("hget", RedisClient::hget)
        .register_fn("hdel", RedisClient::hdel)
        .register_fn("hexists", RedisClient::hexists)
        .register_fn("hlen", RedisClient::hlen)
        .register_fn("hkeys", RedisClient::hkeys)
        .register_fn("hvals", RedisClient::hvals)
        .register_fn("hgetall", RedisClient::hgetall);
}
