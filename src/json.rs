//! Redis JSON operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn json_set(&mut self, key: &str, path: &str, value: &str) -> Dynamic {
        self.cmd(
            "JSON.SET",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
                Dynamic::from(value.to_string()),
            ],
        )
    }

    pub fn json_get(&mut self, key: &str, paths: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(paths);
        self.cmd("JSON.GET", args)
    }

    pub fn json_del(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.DEL",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }

    pub fn json_type(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.TYPE",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }

    pub fn json_strlen(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.STRLEN",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }

    pub fn json_arrappend(&mut self, key: &str, path: &str, values: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(path.to_string()),
        ];
        args.extend(values);
        self.cmd("JSON.ARRAPPEND", args)
    }

    pub fn json_arrindex(&mut self, key: &str, path: &str, value: &str) -> Dynamic {
        self.cmd(
            "JSON.ARRINDEX",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
                Dynamic::from(value.to_string()),
            ],
        )
    }

    pub fn json_arrlen(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.ARRLEN",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }

    pub fn json_arrpop(&mut self, key: &str, path: &str, index: i64) -> Dynamic {
        self.cmd(
            "JSON.ARRPOP",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
                Dynamic::from(index),
            ],
        )
    }

    pub fn json_numincrby(&mut self, key: &str, path: &str, value: f64) -> Dynamic {
        self.cmd(
            "JSON.NUMINCRBY",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
                Dynamic::from(value),
            ],
        )
    }

    pub fn json_objkeys(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.OBJKEYS",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }

    pub fn json_objlen(&mut self, key: &str, path: &str) -> Dynamic {
        self.cmd(
            "JSON.OBJLEN",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(path.to_string()),
            ],
        )
    }
}

pub fn register_json_methods(engine: &mut Engine) {
    engine
        .register_fn("json_set", RedisClient::json_set)
        .register_fn("json_get", RedisClient::json_get)
        .register_fn("json_del", RedisClient::json_del)
        .register_fn("json_type", RedisClient::json_type)
        .register_fn("json_strlen", RedisClient::json_strlen)
        .register_fn("json_arrappend", RedisClient::json_arrappend)
        .register_fn("json_arrindex", RedisClient::json_arrindex)
        .register_fn("json_arrlen", RedisClient::json_arrlen)
        .register_fn("json_arrpop", RedisClient::json_arrpop)
        .register_fn("json_numincrby", RedisClient::json_numincrby)
        .register_fn("json_objkeys", RedisClient::json_objkeys)
        .register_fn("json_objlen", RedisClient::json_objlen);
}
