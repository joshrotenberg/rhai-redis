//! Redis Search operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn ft_create(&mut self, index: &str, schema: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(index.to_string())];
        args.extend(schema);
        self.cmd("FT.CREATE", args)
    }

    pub fn ft_search(&mut self, index: &str, query: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(index.to_string()),
            Dynamic::from(query.to_string()),
        ];
        args.extend(options);
        self.cmd("FT.SEARCH", args)
    }

    pub fn ft_aggregate(&mut self, index: &str, query: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(index.to_string()),
            Dynamic::from(query.to_string()),
        ];
        args.extend(options);
        self.cmd("FT.AGGREGATE", args)
    }

    pub fn ft_info(&mut self, index: &str) -> Dynamic {
        self.cmd("FT.INFO", vec![Dynamic::from(index.to_string())])
    }

    pub fn ft_dropindex(&mut self, index: &str) -> Dynamic {
        self.cmd("FT.DROPINDEX", vec![Dynamic::from(index.to_string())])
    }

    pub fn ft_explain(&mut self, index: &str, query: &str) -> Dynamic {
        self.cmd(
            "FT.EXPLAIN",
            vec![
                Dynamic::from(index.to_string()),
                Dynamic::from(query.to_string()),
            ],
        )
    }

    pub fn ft_tagvals(&mut self, index: &str, field: &str) -> Dynamic {
        self.cmd(
            "FT.TAGVALS",
            vec![
                Dynamic::from(index.to_string()),
                Dynamic::from(field.to_string()),
            ],
        )
    }

pub fn register_search_methods(engine: &mut Engine) {
    engine
        .register_fn("ft_create", RedisClient::ft_create)
        .register_fn("ft_search", RedisClient::ft_search)
        .register_fn("ft_aggregate", RedisClient::ft_aggregate)
        .register_fn("ft_info", RedisClient::ft_info)
        .register_fn("ft_dropindex", RedisClient::ft_dropindex)
        .register_fn("ft_explain", RedisClient::ft_explain)
        .register_fn("ft_tagvals", RedisClient::ft_tagvals);
}

    // Additional search commands

    pub fn ft_cursor_read(&mut self, index: &str, cursor_id: i64, count: Option<i64>) -> Dynamic {
        let mut args = vec![
            Dynamic::from("READ"),
            Dynamic::from(index.to_string()),
            Dynamic::from(cursor_id),
        ];
        if let Some(c) = count {
            args.push(Dynamic::from("COUNT"));
            args.push(Dynamic::from(c));
        }
        self.cmd("FT.CURSOR", args)
    }

    pub fn ft_cursor_del(&mut self, index: &str, cursor_id: i64) -> Dynamic {
        self.cmd(
            "FT.CURSOR",
            vec![
                Dynamic::from("DEL"),
                Dynamic::from(index.to_string()),
                Dynamic::from(cursor_id),
            ],
        )
    }

    pub fn ft_config_set(&mut self, option: &str, value: &str) -> Dynamic {
        self.cmd(
            "FT.CONFIG",
            vec![
                Dynamic::from("SET"),
                Dynamic::from(option.to_string()),
                Dynamic::from(value.to_string()),
            ],
        )
    }

    pub fn ft_config_get(&mut self, option: &str) -> Dynamic {
        self.cmd(
            "FT.CONFIG",
            vec![
                Dynamic::from("GET"),
                Dynamic::from(option.to_string()),
            ],
        )
    }

    pub fn ft_synupdate(&mut self, index: &str, synonym_group_id: &str, terms: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(index.to_string()),
            Dynamic::from(synonym_group_id.to_string()),
        ];
        args.extend(terms);
        self.cmd("FT.SYNUPDATE", args)
    }

    pub fn ft_syndump(&mut self, index: &str) -> Dynamic {
        self.cmd("FT.SYNDUMP", vec![Dynamic::from(index.to_string())])
    }

    pub fn ft_spellcheck(&mut self, index: &str, query: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(index.to_string()),
            Dynamic::from(query.to_string()),
        ];
        args.extend(options);
        self.cmd("FT.SPELLCHECK", args)
    }

    pub fn ft_dictadd(&mut self, dict: &str, terms: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(dict.to_string())];
        args.extend(terms);
        self.cmd("FT.DICTADD", args)
    }

    pub fn ft_dictdel(&mut self, dict: &str, terms: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(dict.to_string())];
        args.extend(terms);
        self.cmd("FT.DICTDEL", args)
    }

    pub fn ft_dictdump(&mut self, dict: &str) -> Dynamic {
        self.cmd("FT.DICTDUMP", vec![Dynamic::from(dict.to_string())])
    }

    pub fn ft_sugadd(&mut self, key: &str, string: &str, score: f64, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(string.to_string()),
            Dynamic::from(score),
        ];
        args.extend(options);
        self.cmd("FT.SUGADD", args)
    }

    pub fn ft_sugget(&mut self, key: &str, prefix: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(prefix.to_string()),
        ];
        args.extend(options);
        self.cmd("FT.SUGGET", args)
    }

    pub fn ft_sugdel(&mut self, key: &str, string: &str) -> Dynamic {
        self.cmd(
            "FT.SUGDEL",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(string.to_string()),
            ],
        )
    }

    pub fn ft_suglen(&mut self, key: &str) -> Dynamic {
        self.cmd("FT.SUGLEN", vec![Dynamic::from(key.to_string())])
    }
}


pub fn register_search_methods(engine: &mut Engine) {
    engine
        .register_fn("ft_create", RedisClient::ft_create)
        .register_fn("ft_search", RedisClient::ft_search)
        .register_fn("ft_aggregate", RedisClient::ft_aggregate)
        .register_fn("ft_info", RedisClient::ft_info)
        .register_fn("ft_dropindex", RedisClient::ft_dropindex)
        .register_fn("ft_explain", RedisClient::ft_explain)
        .register_fn("ft_tagvals", RedisClient::ft_tagvals)
        .register_fn("ft_cursor_read", RedisClient::ft_cursor_read)
        .register_fn("ft_cursor_del", RedisClient::ft_cursor_del)
        .register_fn("ft_config_set", RedisClient::ft_config_set)
        .register_fn("ft_config_get", RedisClient::ft_config_get)
        .register_fn("ft_synupdate", RedisClient::ft_synupdate)
        .register_fn("ft_syndump", RedisClient::ft_syndump)
        .register_fn("ft_spellcheck", RedisClient::ft_spellcheck)
        .register_fn("ft_dictadd", RedisClient::ft_dictadd)
        .register_fn("ft_dictdel", RedisClient::ft_dictdel)
        .register_fn("ft_dictdump", RedisClient::ft_dictdump)
        .register_fn("ft_sugadd", RedisClient::ft_sugadd)
        .register_fn("ft_sugget", RedisClient::ft_sugget)
        .register_fn("ft_sugdel", RedisClient::ft_sugdel)
        .register_fn("ft_suglen", RedisClient::ft_suglen);
}
