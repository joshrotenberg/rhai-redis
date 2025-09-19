//! Redis-enabled Rhai engine

use crate::{RedisClient, Result};
use rhai::{Engine, Scope};

/// A Rhai engine configured for Redis operations
pub struct RedisEngine {
    engine: Engine,
    client: Option<RedisClient>,
}

impl RedisEngine {
    /// Create a new Redis-enabled Rhai engine
    pub fn new() -> Self {
        let engine = create_redis_engine().expect("Failed to create engine");
        Self {
            engine,
            client: None,
        }
    }

    /// Set the Redis client for this engine
    pub fn set_redis_client(&mut self, client: RedisClient) {
        self.client = Some(client);
    }

    /// Run a script with the configured Redis client
    pub fn run(&mut self, script: &str) -> Result<()> {
        let client = self.client.as_ref()
            .ok_or_else(|| crate::Error::Connection("No Redis client configured".into()))?;

        let mut scope = Scope::new();
        scope.push("redis", client.clone());

        self.engine
            .run_with_scope(&mut scope, script)
            .map_err(|e| crate::Error::Script(e.to_string()))?;

        Ok(())
    }

    /// Run a script with variables
    pub fn run_with_variables(&mut self, script: &str, vars: Vec<(String, String)>) -> Result<()> {
        let client = self.client.as_ref()
            .ok_or_else(|| crate::Error::Connection("No Redis client configured".into()))?;

        let mut scope = Scope::new();
        scope.push("redis", client.clone());

        for (name, value) in vars {
            scope.push(name, value);
        }

        self.engine
            .run_with_scope(&mut scope, script)
            .map_err(|e| crate::Error::Script(e.to_string()))?;

        Ok(())
    }

    /// Get a reference to the underlying Rhai engine for customization
    pub fn engine(&mut self) -> &mut Engine {
        &mut self.engine
    }
}

/// Create and configure a Rhai engine with Redis commands
pub fn create_redis_engine() -> Result<Engine> {
    let mut engine = Engine::new();

    // Security limits
    engine.set_max_expr_depths(100, 100);
    engine.set_max_operations(1_000_000);

    // Set up print callback
    engine.on_print(|s| {
        println!("{}", s);
    });

    // Register the RedisClient type
    engine.register_type::<RedisClient>();

    // Register all Redis methods from each module
    crate::strings::register_string_methods(&mut engine);
    crate::keys::register_key_methods(&mut engine);
    crate::lists::register_list_methods(&mut engine);
    crate::hashes::register_hash_methods(&mut engine);
    crate::sets::register_set_methods(&mut engine);
    crate::sorted_sets::register_sorted_set_methods(&mut engine);
    crate::search::register_search_methods(&mut engine);
    crate::json::register_json_methods(&mut engine);
    crate::streams::register_stream_methods(&mut engine);
    crate::pubsub::register_pubsub_methods(&mut engine);
    crate::transactions::register_transaction_methods(&mut engine);
    crate::generic::register_generic_methods(&mut engine);
    
    // Register new modules
    crate::bitmap::register_bitmap_methods(&mut engine);
    crate::bloom::register_bloom_methods(&mut engine);
    crate::hyperloglog::register_hyperloglog_methods(&mut engine);
    crate::geo::register_geo_methods(&mut engine);

    // Register utility functions
    crate::utils::register_utility_functions(&mut engine);

    Ok(engine)
}
