//! Transaction example for rhai-redis

use redis::Client;
use rhai_redis::{RedisClient, RedisEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::open("redis://localhost:6379")?;
    let conn = client.get_connection()?;

    let mut engine = RedisEngine::new();
    engine.set_redis_client(RedisClient::new(conn));

    engine.run(
        r#"
        print("=== Transaction Example ===");

        // Set initial values
        redis.set("balance:alice", "100");
        redis.set("balance:bob", "50");

        print("Initial balances:");
        print("Alice: " + redis.get("balance:alice"));
        print("Bob: " + redis.get("balance:bob"));

        // Start a transaction
        redis.multi();

        // Transfer 30 from Alice to Bob
        redis.decrby("balance:alice", 30);
        redis.incrby("balance:bob", 30);

        // Execute the transaction
        let results = redis.exec();
        print("Transaction executed: " + results.len().to_string() + " operations");

        print("\nFinal balances:");
        print("Alice: " + redis.get("balance:alice"));
        print("Bob: " + redis.get("balance:bob"));

        // Example of discarding a transaction
        print("\n=== Discard Example ===");
        redis.multi();
        redis.set("temp", "value");
        redis.discard();
        
        // This should return nothing (unit)
        let temp = redis.get("temp");
        if temp == () {
            print("Transaction was discarded, temp key not set");
        }
    "#,
    )?;

    Ok(())
}
