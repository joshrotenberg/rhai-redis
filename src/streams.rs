//! Stream operations for Redis Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn xadd(&mut self, key: &str, id: &str, fields: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(id.to_string()),
        ];
        args.extend(fields);
        self.cmd("XADD", args)
    }

    pub fn xread(&mut self, count: i64, streams: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from("COUNT"),
            Dynamic::from(count),
            Dynamic::from("STREAMS"),
        ];
        args.extend(streams);
        self.cmd("XREAD", args)
    }

    pub fn xrange(&mut self, key: &str, start: &str, end: &str) -> Dynamic {
        self.cmd(
            "XRANGE",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(start.to_string()),
                Dynamic::from(end.to_string()),
            ],
        )
    }

    pub fn xrevrange(&mut self, key: &str, end: &str, start: &str) -> Dynamic {
        self.cmd(
            "XREVRANGE",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(end.to_string()),
                Dynamic::from(start.to_string()),
            ],
        )
    }

    pub fn xlen(&mut self, key: &str) -> Dynamic {
        self.cmd("XLEN", vec![Dynamic::from(key.to_string())])
    }

    pub fn xdel(&mut self, key: &str, ids: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(ids);
        self.cmd("XDEL", args)
    }

    pub fn xtrim(&mut self, key: &str, strategy: &str, threshold: i64) -> Dynamic {
        self.cmd(
            "XTRIM",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(strategy.to_string()),
                Dynamic::from(threshold),
            ],
        )
    }

    pub fn xgroup_create(&mut self, key: &str, group: &str, id: &str) -> Dynamic {
        self.cmd(
            "XGROUP",
            vec![
                Dynamic::from("CREATE"),
                Dynamic::from(key.to_string()),
                Dynamic::from(group.to_string()),
                Dynamic::from(id.to_string()),
            ],
        )
    }

    pub fn xgroup_destroy(&mut self, key: &str, group: &str) -> Dynamic {
        self.cmd(
            "XGROUP",
            vec![
                Dynamic::from("DESTROY"),
                Dynamic::from(key.to_string()),
                Dynamic::from(group.to_string()),
            ],
        )
    }

    pub fn xreadgroup(
        &mut self,
        group: &str,
        consumer: &str,
        count: i64,
        streams: Vec<Dynamic>,
    ) -> Dynamic {
        let mut args = vec![
            Dynamic::from("GROUP"),
            Dynamic::from(group.to_string()),
            Dynamic::from(consumer.to_string()),
            Dynamic::from("COUNT"),
            Dynamic::from(count),
            Dynamic::from("STREAMS"),
        ];
        args.extend(streams);
        self.cmd("XREADGROUP", args)
    }
}

pub fn register_stream_methods(engine: &mut Engine) {
    engine
        .register_fn("xadd", RedisClient::xadd)
        .register_fn("xread", RedisClient::xread)
        .register_fn("xrange", RedisClient::xrange)
        .register_fn("xrevrange", RedisClient::xrevrange)
        .register_fn("xlen", RedisClient::xlen)
        .register_fn("xdel", RedisClient::xdel)
        .register_fn("xtrim", RedisClient::xtrim)
        .register_fn("xgroup_create", RedisClient::xgroup_create)
        .register_fn("xgroup_destroy", RedisClient::xgroup_destroy)
        .register_fn("xreadgroup", RedisClient::xreadgroup);
}
