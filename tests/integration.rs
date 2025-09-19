#[cfg(test)]
mod integration_tests {
    use rhai_redis::{RedisEngine, RedisClient};
    use redis::Client;
    use serial_test::serial;

    fn get_redis_connection() -> redis::Connection {
        let client = Client::open("redis://localhost:6379")
            .expect("Failed to create client");
        client.get_connection()
            .expect("Failed to connect to Redis")
    }

    #[test]
    #[serial]
    fn test_basic_operations() {
        let conn = get_redis_connection();
        let mut engine = RedisEngine::new();
        engine.set_redis_client(RedisClient::new(conn));

        engine.run(r#"
            redis.set("test:key", "test_value");
            let value = redis.get("test:key");
            if value != "test_value" {
                throw "Value mismatch: " + value;
            }
            redis.del("test:key");
        "#).expect("Script failed");
    }

    #[test]
    #[serial]
    fn test_list_operations() {
        let conn = get_redis_connection();
        let mut engine = RedisEngine::new();
        engine.set_redis_client(RedisClient::new(conn));

        engine.run(r#"
            redis.del("test:list");
            redis.lpush("test:list", "item1");
            redis.lpush("test:list", "item2");
            
            let len = redis.llen("test:list");
            if len != 2 {
                throw "List length wrong: " + len.to_string();
            }
            
            redis.del("test:list");
        "#).expect("Script failed");
    }

    #[test]
    #[serial]
    fn test_hash_operations() {
        let conn = get_redis_connection();
        let mut engine = RedisEngine::new();
        engine.set_redis_client(RedisClient::new(conn));

        engine.run(r#"
            redis.del("test:hash");
            redis.hset("test:hash", "field1", "value1");
            redis.hset("test:hash", "field2", "value2");
            
            let value = redis.hget("test:hash", "field1");
            if value != "value1" {
                throw "Hash value wrong: " + value;
            }
            
            let len = redis.hlen("test:hash");
            if len != 2 {
                throw "Hash length wrong: " + len.to_string();
            }
            
            redis.del("test:hash");
        "#).expect("Script failed");
    }
}
