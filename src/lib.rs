//! Redis scripting support for Rhai
//!
//! This crate provides a clean integration between Rhai (an embedded scripting language)
//! and Redis, allowing you to write Redis scripts in a safe, sandboxed environment.
//!
//! # Example
//!
//! ```no_run
//! use rhai_redis::{RedisEngine, RedisClient};
//! use redis::Client;
//!
//! let client = Client::open("redis://localhost").unwrap();
//! let conn = client.get_connection().unwrap();
//!
//! let mut engine = RedisEngine::new();
//! engine.set_redis_client(RedisClient::new(conn));
//!
//! engine.run(r#"
//!     redis.set("key", "value");
//!     let value = redis.get("key");
//!     print("Got: " + value);
//! "#).unwrap();
//! ```

pub mod bitmap;
pub mod bloom;
pub mod client;
pub mod generic;
pub mod geo;
pub mod hashes;
pub mod hyperloglog;
pub mod json;
pub mod keys;
pub mod lists;
pub mod pubsub;
pub mod search;
pub mod sets;
pub mod sorted_sets;
pub mod streams;
pub mod strings;
pub mod transactions;
pub mod utils;

mod engine;
mod error;

pub use client::RedisClient;
pub use engine::{create_redis_engine, RedisEngine};
pub use error::{Error, Result};

// Re-export rhai types that users might need
pub use rhai::{Dynamic, Engine, Scope};
