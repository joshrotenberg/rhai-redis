//! Redis Bitmap operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Set bit at offset
    pub fn setbit(&mut self, key: &str, offset: i64, value: i64) -> Dynamic {
        self.cmd(
            "SETBIT",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(offset),
                Dynamic::from(value),
            ],
        )
    }

    /// Get bit at offset
    pub fn getbit(&mut self, key: &str, offset: i64) -> Dynamic {
        self.cmd(
            "GETBIT",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(offset),
            ],
        )
    }

    /// Count set bits in range
    pub fn bitcount(&mut self, key: &str, start: Option<i64>, end: Option<i64>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        if let Some(s) = start {
            args.push(Dynamic::from(s));
            if let Some(e) = end {
                args.push(Dynamic::from(e));
            }
        }
        self.cmd("BITCOUNT", args)
    }

    /// Bitwise operation between strings
    pub fn bitop(&mut self, operation: &str, destkey: &str, keys: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(operation.to_string()),
            Dynamic::from(destkey.to_string()),
        ];
        args.extend(keys);
        self.cmd("BITOP", args)
    }

    /// Find first bit set or clear
    pub fn bitpos(&mut self, key: &str, bit: i64, start: Option<i64>, end: Option<i64>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(bit),
        ];
        if let Some(s) = start {
            args.push(Dynamic::from(s));
            if let Some(e) = end {
                args.push(Dynamic::from(e));
            }
        }
        self.cmd("BITPOS", args)
    }

    /// Bitfield operations
    pub fn bitfield(&mut self, key: &str, operations: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(operations);
        self.cmd("BITFIELD", args)
    }

    /// Read-only bitfield operations
    pub fn bitfield_ro(&mut self, key: &str, operations: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(operations);
        self.cmd("BITFIELD_RO", args)
    }
}

/// Register Bitmap methods with Rhai engine
pub fn register_bitmap_methods(engine: &mut Engine) {
    engine
        .register_fn("setbit", |client: &mut RedisClient, key: &str, offset: i64, value: i64| {
            client.setbit(key, offset, value)
        })
        .register_fn("getbit", |client: &mut RedisClient, key: &str, offset: i64| {
            client.getbit(key, offset)
        })
        .register_fn("bitcount", |client: &mut RedisClient, key: &str| {
            client.bitcount(key, None, None)
        })
        .register_fn("bitcount_range", |client: &mut RedisClient, key: &str, start: i64, end: i64| {
            client.bitcount(key, Some(start), Some(end))
        })
        .register_fn("bitop", |client: &mut RedisClient, operation: &str, destkey: &str, keys: Vec<Dynamic>| {
            client.bitop(operation, destkey, keys)
        })
        .register_fn("bitpos", |client: &mut RedisClient, key: &str, bit: i64| {
            client.bitpos(key, bit, None, None)
        })
        .register_fn("bitpos_range", |client: &mut RedisClient, key: &str, bit: i64, start: i64, end: i64| {
            client.bitpos(key, bit, Some(start), Some(end))
        })
        .register_fn("bitfield", |client: &mut RedisClient, key: &str, operations: Vec<Dynamic>| {
            client.bitfield(key, operations)
        })
        .register_fn("bitfield_ro", |client: &mut RedisClient, key: &str, operations: Vec<Dynamic>| {
            client.bitfield_ro(key, operations)
        });
}
