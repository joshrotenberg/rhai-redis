//! Basic example of using rhai-redis

use rhai_redis::{RedisEngine, RedisClient};
use redis::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Redis
    let client = Client::open("redis://localhost:6379")?;
    let conn = client.get_connection()?;

    // Create engine and set Redis client
    let mut engine = RedisEngine::new();
    engine.set_redis_client(RedisClient::new(conn));

    // Run a simple script
    engine.run(r#"
        // Set some values
        redis.set("name", "Alice");
        redis.set("age", "30");
        redis.set("city", "Wonderland");

        // Get and print values
        let name = redis.get("name");
        let age = redis.get("age");
        let city = redis.get("city");

        print("Name: " + name);
        print("Age: " + age);
        print("City: " + city);

        // Use a list
        redis.lpush("colors", "red");
        redis.lpush("colors", "green");
        redis.lpush("colors", "blue");

        let colors = redis.lrange("colors", 0, -1);
        print("Colors: " + colors);

        // Use a hash
        redis.hset("user:1", "username", "alice");
        redis.hset("user:1", "email", "alice@example.com");
        
        let username = redis.hget("user:1", "username");
        print("Username: " + username);

        // Increment a counter
        redis.set("counter", "0");
        let count = redis.incr("counter");
        print("Counter: " + count.to_string());
    "#)?;

    Ok(())
}
