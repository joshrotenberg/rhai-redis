#[cfg(test)]
mod hyperloglog_tests {
    use rhai_redis::{RedisEngine, RedisClient};
    use redis::Client;

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
    fn test_hyperloglog_basic() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add elements to HyperLogLog
            let elements = ["a", "b", "c", "d", "e"];
            let added = redis.pfadd("hll:test", elements);
            print("Added new elements: " + added.to_string());
            
            // Add duplicate - should return 0
            let duplicate = redis.pfadd("hll:test", ["a", "b"]);
            print("Added duplicates: " + duplicate.to_string());
            
            // Count unique elements
            let count = redis.pfcount(["hll:test"]);
            print("Unique count: " + count.to_string());
        "#;
        
        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_hyperloglog_merge() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Create two HyperLogLogs
            redis.pfadd("hll:set1", ["a", "b", "c"]);
            redis.pfadd("hll:set2", ["c", "d", "e"]);
            
            // Count each
            let count1 = redis.pfcount(["hll:set1"]);
            let count2 = redis.pfcount(["hll:set2"]);
            print("Set1 count: " + count1.to_string());
            print("Set2 count: " + count2.to_string());
            
            // Merge them
            redis.pfmerge("hll:merged", ["hll:set1", "hll:set2"]);
            
            // Count merged (should be ~5 unique)
            let merged_count = redis.pfcount(["hll:merged"]);
            print("Merged count: " + merged_count.to_string());
        "#;
        
        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_hyperloglog_large_dataset() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add many unique elements
            for i in 0..1000 {
                redis.pfadd("hll:large", ["user:" + i.to_string()]);
            }
            
            // Count should be close to 1000
            let count = redis.pfcount(["hll:large"]);
            print("Count of 1000 unique items: " + count.to_string());
            
            // HyperLogLog uses very little memory even with many items
            let memory = redis.cmd("MEMORY", ["USAGE", "hll:large"]);
            print("Memory usage: " + memory.to_string() + " bytes");
        "#;
        
        engine.run(script).expect("Script failed");
    }
}
