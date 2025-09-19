//! Generic command execution and utility functions for Redis Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    pub fn cmd(&mut self, command: &str, args: Vec<Dynamic>) -> Dynamic {
        let mut conn = self.conn.lock().unwrap();
        let mut redis_cmd = redis::cmd(command);

        for arg in args {
            if let Ok(s) = arg.clone().into_immutable_string() {
                redis_cmd.arg(s.as_str());
            } else if let Ok(i) = arg.as_int() {
                redis_cmd.arg(i);
            } else if let Ok(f) = arg.as_float() {
                redis_cmd.arg(f);
            } else {
                redis_cmd.arg(arg.to_string());
            }
        }

        match redis_cmd.query::<redis::Value>(&mut *conn) {
            Ok(value) => redis_value_to_dynamic(value),
            Err(e) => {
                eprintln!("Redis error in cmd: {}", e);
                Dynamic::UNIT
            }
        }
    }
}

pub fn redis_value_to_dynamic(value: redis::Value) -> Dynamic {
    match value {
        redis::Value::Nil => Dynamic::UNIT,
        redis::Value::Int(i) => Dynamic::from(i),
        redis::Value::BulkString(bytes) => String::from_utf8_lossy(&bytes).to_string().into(),
        redis::Value::Array(arr) => {
            let vec: Vec<Dynamic> = arr.into_iter().map(redis_value_to_dynamic).collect();
            vec.into()
        }
        redis::Value::SimpleString(s) => s.into(),
        redis::Value::Okay => Dynamic::from("OK"),
        redis::Value::Map(map) => {
            let mut rhai_map = rhai::Map::new();
            for (k, v) in map {
                let key = match k {
                    redis::Value::BulkString(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                    redis::Value::SimpleString(s) => s,
                    _ => format!("{:?}", k),
                };
                rhai_map.insert(key.into(), redis_value_to_dynamic(v));
            }
            rhai_map.into()
        }
        _ => format!("{:?}", value).into(),
    }
}

pub fn register_generic_methods(engine: &mut Engine) {
    engine.register_fn("cmd", RedisClient::cmd);
}
