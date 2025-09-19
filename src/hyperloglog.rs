//! Redis HyperLogLog operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Add elements to HyperLogLog
    pub fn pfadd(&mut self, key: &str, elements: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(elements);
        self.cmd("PFADD", args)
    }

    /// Count unique elements in HyperLogLog
    pub fn pfcount(&mut self, keys: Vec<Dynamic>) -> Dynamic {
        self.cmd("PFCOUNT", keys)
    }

    /// Merge multiple HyperLogLogs
    pub fn pfmerge(&mut self, destkey: &str, sourcekeys: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(destkey.to_string())];
        args.extend(sourcekeys);
        self.cmd("PFMERGE", args)
    }

    /// Debug HyperLogLog internals
    pub fn pfdebug(&mut self, subcommand: &str, key: &str) -> Dynamic {
        self.cmd(
            "PFDEBUG",
            vec![
                Dynamic::from(subcommand.to_string()),
                Dynamic::from(key.to_string()),
            ],
        )
    }

    /// Get HyperLogLog representation
    pub fn pfselftest(&mut self) -> Dynamic {
        self.cmd("PFSELFTEST", vec![])
    }
}

/// Register HyperLogLog methods with Rhai engine
pub fn register_hyperloglog_methods(engine: &mut Engine) {
    engine
        .register_fn(
            "pfadd",
            |client: &mut RedisClient, key: &str, elements: Vec<Dynamic>| {
                client.pfadd(key, elements)
            },
        )
        .register_fn("pfcount", |client: &mut RedisClient, keys: Vec<Dynamic>| {
            client.pfcount(keys)
        })
        .register_fn(
            "pfmerge",
            |client: &mut RedisClient, destkey: &str, sourcekeys: Vec<Dynamic>| {
                client.pfmerge(destkey, sourcekeys)
            },
        )
        .register_fn(
            "pfdebug",
            |client: &mut RedisClient, subcommand: &str, key: &str| client.pfdebug(subcommand, key),
        )
        .register_fn("pfselftest", |client: &mut RedisClient| client.pfselftest());
}
