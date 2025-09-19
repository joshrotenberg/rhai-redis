//! Hash operations for Redis Rhai integration
//!
//! This module provides Redis hash commands for use in Rhai scripts.

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Set the string value of a hash field.
    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hset::<_, _, _, i64>(key, field, value).unwrap_or(0)
    }

    /// Get the value of a hash field.
    pub fn hget(&mut self, key: &str, field: &str) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        match conn.hget::<_, _, Option<String>>(key, field) {
            Ok(Some(val)) => Dynamic::from(val),
            Ok(None) => Dynamic::UNIT,
            Err(_) => Dynamic::UNIT,
        }
    }

    /// Delete one or more hash fields.
    pub fn hdel(&mut self, key: &str, field: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel::<_, _, i64>(key, field).unwrap_or(0)
    }

    /// Check if a hash field exists.
    pub fn hexists(&mut self, key: &str, field: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.hexists::<_, _, bool>(key, field).unwrap_or(false)
    }

    /// Get the number of fields in a hash.
    pub fn hlen(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.hlen::<_, i64>(key).unwrap_or(0)
    }

    /// Get all field names in a hash.
    pub fn hkeys(&mut self, key: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        conn.hkeys::<_, Vec<String>>(key)
            .unwrap_or_default()
            .into_iter()
            .map(Dynamic::from)
            .collect()
    }

    /// Get all values in a hash.
    pub fn hvals(&mut self, key: &str) -> Vec<Dynamic> {
        let mut conn = self.conn.lock().unwrap();
        conn.hvals::<_, Vec<String>>(key)
            .unwrap_or_default()
            .into_iter()
            .map(Dynamic::from)
            .collect()
    }

    /// Get all fields and values in a hash.
    pub fn hgetall(&mut self, key: &str) -> rhai::Map {
        let mut conn = self.conn.lock().unwrap();
        let result: Vec<(String, String)> = conn.hgetall(key).unwrap_or_default();
        
        let mut map = rhai::Map::new();
        for (k, v) in result {
            map.insert(k.into(), Dynamic::from(v));
        }
        map
    }
}

/// Register hash methods with the Rhai engine
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
