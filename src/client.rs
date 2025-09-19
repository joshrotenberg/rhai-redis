//! Redis client for Rhai scripting

use redis::Connection;
use std::sync::{Arc, Mutex};

/// Thread-safe Redis client for Rhai scripting
#[derive(Clone)]
pub struct RedisClient {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl RedisClient {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}
