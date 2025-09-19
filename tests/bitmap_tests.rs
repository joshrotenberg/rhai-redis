#[cfg(test)]
mod bitmap_tests {
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
    #[ignore]
    fn test_bitmap_basic() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Set bits
            redis.setbit("bitmap:test", 0, 1);
            redis.setbit("bitmap:test", 2, 1);
            redis.setbit("bitmap:test", 5, 1);
            
            // Get bits
            let bit0 = redis.getbit("bitmap:test", 0);
            let bit1 = redis.getbit("bitmap:test", 1);
            let bit2 = redis.getbit("bitmap:test", 2);
            
            print("Bit 0: " + bit0.to_string());
            print("Bit 1: " + bit1.to_string());
            print("Bit 2: " + bit2.to_string());
            
            // Count set bits
            let count = redis.bitcount("bitmap:test");
            print("Total set bits: " + count.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_bitmap_operations() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Create two bitmaps
            redis.setbit("bitmap:a", 0, 1);
            redis.setbit("bitmap:a", 2, 1);
            redis.setbit("bitmap:a", 4, 1);
            
            redis.setbit("bitmap:b", 1, 1);
            redis.setbit("bitmap:b", 2, 1);
            redis.setbit("bitmap:b", 3, 1);
            
            // AND operation
            let and_result = redis.bitop("AND", "bitmap:and", ["bitmap:a", "bitmap:b"]);
            print("AND result length: " + and_result.to_string());
            
            let and_count = redis.bitcount("bitmap:and");
            print("AND set bits: " + and_count.to_string());
            
            // OR operation
            let or_result = redis.bitop("OR", "bitmap:or", ["bitmap:a", "bitmap:b"]);
            print("OR result length: " + or_result.to_string());
            
            let or_count = redis.bitcount("bitmap:or");
            print("OR set bits: " + or_count.to_string());
            
            // XOR operation
            let xor_result = redis.bitop("XOR", "bitmap:xor", ["bitmap:a", "bitmap:b"]);
            let xor_count = redis.bitcount("bitmap:xor");
            print("XOR set bits: " + xor_count.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_bitmap_position() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Create bitmap with pattern
            redis.setbit("bitmap:pos", 2, 1);
            redis.setbit("bitmap:pos", 5, 1);
            redis.setbit("bitmap:pos", 10, 1);
            
            // Find first set bit
            let first_set = redis.bitpos("bitmap:pos", 1);
            print("First set bit at position: " + first_set.to_string());
            
            // Find first clear bit
            let first_clear = redis.bitpos("bitmap:pos", 0);
            print("First clear bit at position: " + first_clear.to_string());
            
            // Find in range
            let in_range = redis.bitpos_range("bitmap:pos", 1, 3, 15);
            print("First set bit in range [3,15]: " + in_range.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_bitmap_counting() {
        let mut engine = setup();

        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Track daily active users
            for user_id in 1..100 {
                if user_id % 3 == 0 {
                    redis.setbit("users:2024-01-01", user_id, 1);
                }
                if user_id % 2 == 0 {
                    redis.setbit("users:2024-01-02", user_id, 1);
                }
            }
            
            // Count active users per day
            let day1_count = redis.bitcount("users:2024-01-01");
            let day2_count = redis.bitcount("users:2024-01-02");
            
            print("Day 1 active users: " + day1_count.to_string());
            print("Day 2 active users: " + day2_count.to_string());
            
            // Users active both days (AND)
            redis.bitop("AND", "users:both_days", ["users:2024-01-01", "users:2024-01-02"]);
            let both_days = redis.bitcount("users:both_days");
            print("Active both days: " + both_days.to_string());
            
            // Users active either day (OR)
            redis.bitop("OR", "users:any_day", ["users:2024-01-01", "users:2024-01-02"]);
            let any_day = redis.bitcount("users:any_day");
            print("Active any day: " + any_day.to_string());
        "#;

        engine.run(script).expect("Script failed");
    }
}
