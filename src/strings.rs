//! String operations for Redis Rhai integration
//!
//! This module provides Redis string commands for use in Rhai scripts.
//! All methods are available on the `redis` object in scripts.
//!
//! # Example
//! ```rhai
//! redis.set("key", "value");
//! let value = redis.get("key");
//! print(value); // prints: value
//! ```

use crate::client::RedisClient;
use redis::Commands;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Get the value of a key.
    ///
    /// # Rhai Example
    /// ```rhai
    /// let value = redis.get("mykey");
    /// if value != () {
    ///     print("Got: " + value);
    /// } else {
    ///     print("Key not found");
    /// }
    /// ```
    ///
    /// # Returns
    /// - The value as a string if the key exists
    /// - Unit `()` if the key doesn't exist or on error
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

    /// Set a key to hold a string value.
    ///
    /// # Rhai Example
    /// ```rhai
    /// if redis.set("name", "Alice") {
    ///     print("Key set successfully");
    /// }
    /// ```
    ///
    /// # Returns
    /// - `true` if the key was set
    /// - `false` on error
    pub fn set(&mut self, key: &str, value: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.set::<_, _, ()>(key, value).is_ok()
    }

    /// Delete a key.
    ///
    /// # Rhai Example
    /// ```rhai
    /// let deleted = redis.del("mykey");
    /// print("Deleted " + deleted + " key(s)");
    /// ```
    ///
    /// # Returns
    /// The number of keys that were removed
    pub fn del(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.del::<_, i64>(key).unwrap_or(0)
    }

    /// Check if a key exists.
    ///
    /// # Rhai Example
    /// ```rhai
    /// if redis.exists("mykey") {
    ///     print("Key exists");
    /// } else {
    ///     print("Key does not exist");
    /// }
    /// ```
    ///
    /// # Returns
    /// `true` if the key exists, `false` otherwise
    pub fn exists(&mut self, key: &str) -> bool {
        let mut conn = self.conn.lock().unwrap();
        conn.exists::<_, bool>(key).unwrap_or(false)
    }

    /// Increment the integer value of a key by 1.
    ///
    /// # Rhai Example
    /// ```rhai
    /// redis.set("counter", "10");
    /// let new_value = redis.incr("counter");
    /// print("Counter is now: " + new_value); // 11
    /// ```
    ///
    /// # Returns
    /// The value of the key after incrementing
    pub fn incr(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.incr::<_, _, i64>(key, 1).unwrap_or(0)
    }

    /// Increment the integer value of a key by the given amount.
    ///
    /// # Rhai Example
    /// ```rhai
    /// redis.set("counter", "100");
    /// let new_value = redis.incrby("counter", 25);
    /// print("Counter is now: " + new_value); // 125
    /// ```
    ///
    /// # Returns
    /// The value of the key after incrementing
    pub fn incrby(&mut self, key: &str, increment: i64) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.incr::<_, _, i64>(key, increment).unwrap_or(0)
    }

    /// Decrement the integer value of a key by 1.
    ///
    /// # Rhai Example
    /// ```rhai
    /// redis.set("counter", "10");
    /// let new_value = redis.decr("counter");
    /// print("Counter is now: " + new_value); // 9
    /// ```
    ///
    /// # Returns
    /// The value of the key after decrementing
    pub fn decr(&mut self, key: &str) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.decr::<_, _, i64>(key, 1).unwrap_or(0)
    }

    /// Decrement the integer value of a key by the given amount.
    ///
    /// # Rhai Example
    /// ```rhai
    /// redis.set("counter", "100");
    /// let new_value = redis.decrby("counter", 25);
    /// print("Counter is now: " + new_value); // 75
    /// ```
    ///
    /// # Returns
    /// The value of the key after decrementing
    pub fn decrby(&mut self, key: &str, decrement: i64) -> i64 {
        let mut conn = self.conn.lock().unwrap();
        conn.decr::<_, _, i64>(key, decrement).unwrap_or(0)
    }
}

/// Register string methods with the Rhai engine
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
