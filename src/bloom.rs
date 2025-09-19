//! Redis Bloom Filter operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Reserve a new bloom filter
    pub fn bf_reserve(&mut self, key: &str, error_rate: f64, capacity: i64) -> Dynamic {
        self.cmd(
            "BF.RESERVE",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(error_rate),
                Dynamic::from(capacity),
            ],
        )
    }

    /// Add an item to bloom filter
    pub fn bf_add(&mut self, key: &str, item: &str) -> Dynamic {
        self.cmd(
            "BF.ADD",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(item.to_string()),
            ],
        )
    }

    /// Check if item exists in bloom filter
    pub fn bf_exists(&mut self, key: &str, item: &str) -> Dynamic {
        self.cmd(
            "BF.EXISTS",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(item.to_string()),
            ],
        )
    }

    /// Add multiple items to bloom filter
    pub fn bf_madd(&mut self, key: &str, items: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(items);
        self.cmd("BF.MADD", args)
    }

    /// Check if multiple items exist
    pub fn bf_mexists(&mut self, key: &str, items: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(items);
        self.cmd("BF.MEXISTS", args)
    }

    /// Get bloom filter info
    pub fn bf_info(&mut self, key: &str) -> Dynamic {
        self.cmd("BF.INFO", vec![Dynamic::from(key.to_string())])
    }

    /// Insert items with custom options
    pub fn bf_insert(&mut self, key: &str, options: Vec<Dynamic>, items: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(options);
        args.push(Dynamic::from("ITEMS"));
        args.extend(items);
        self.cmd("BF.INSERT", args)
    }
}

/// Register Bloom filter methods with Rhai engine
pub fn register_bloom_methods(engine: &mut Engine) {
    engine
        .register_fn(
            "bf_reserve",
            |client: &mut RedisClient, key: &str, error_rate: f64, capacity: i64| {
                client.bf_reserve(key, error_rate, capacity)
            },
        )
        .register_fn(
            "bf_add",
            |client: &mut RedisClient, key: &str, item: &str| client.bf_add(key, item),
        )
        .register_fn(
            "bf_exists",
            |client: &mut RedisClient, key: &str, item: &str| client.bf_exists(key, item),
        )
        .register_fn(
            "bf_madd",
            |client: &mut RedisClient, key: &str, items: Vec<Dynamic>| client.bf_madd(key, items),
        )
        .register_fn(
            "bf_mexists",
            |client: &mut RedisClient, key: &str, items: Vec<Dynamic>| {
                client.bf_mexists(key, items)
            },
        )
        .register_fn("bf_info", |client: &mut RedisClient, key: &str| {
            client.bf_info(key)
        })
        .register_fn(
            "bf_insert",
            |client: &mut RedisClient, key: &str, options: Vec<Dynamic>, items: Vec<Dynamic>| {
                client.bf_insert(key, options, items)
            },
        );
}
