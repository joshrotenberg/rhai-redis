#[cfg(test)]
mod bloom_tests {
    use redis::Client;
    use rhai_redis::{RedisClient, RedisEngine};

    fn setup() -> RedisEngine {
        let client = Client::open("redis://localhost:6379").expect("Failed to connect");
        let conn = client.get_connection().expect("Failed to get connection");
        let redis_client = RedisClient::new(conn);

        let mut engine = RedisEngine::new();
        engine.set_redis_client(redis_client);
        engine
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_bloom_filter_basic() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Reserve a bloom filter
            redis.bf_reserve("test:bloom", 0.01, 1000);
            
            // Add items
            let added = redis.bf_add("test:bloom", "item1");
            print("Added item1: " + added.to_string());
            
            // Check existence
            let exists = redis.bf_exists("test:bloom", "item1");
            print("item1 exists: " + exists.to_string());
            
            let not_exists = redis.bf_exists("test:bloom", "item99");
            print("item99 exists: " + not_exists.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_bloom_filter_multiple() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            redis.bf_reserve("test:bloom", 0.01, 1000);
            
            // Add multiple items
            let items = ["item1", "item2", "item3"];
            let added = redis.bf_madd("test:bloom", items);
            print("Added multiple: " + added.to_string());
            
            // Check multiple items
            let check_items = ["item1", "item2", "item99"];
            let results = redis.bf_mexists("test:bloom", check_items);
            print("Multiple exists: " + results.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_bloom_filter_info() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            redis.bf_reserve("test:bloom", 0.01, 1000);
            
            // Add some items
            for i in 0..10 {
                redis.bf_add("test:bloom", "item" + i.to_string());
            }
            
            // Get info
            let info = redis.bf_info("test:bloom");
            print("Bloom filter info: " + info.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }
}
