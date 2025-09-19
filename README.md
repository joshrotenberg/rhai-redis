# rhai-redis

Redis scripting support for [Rhai](https://rhai.rs) - an embedded scripting language for Rust.

## Overview

`rhai-redis` provides a clean, intuitive interface for using Redis commands within Rhai scripts. It wraps the `redis-rs` crate and exposes Redis operations through an object-oriented API.

## Features

- 🚀 **Full Redis Command Coverage**: Strings, Lists, Hashes, Sets, Sorted Sets, Streams, Transactions, and more
- 🛡️ **Safe & Sandboxed**: Scripts run in a controlled environment with configurable limits
- 🎯 **Object-Oriented API**: Natural `redis.set()`, `redis.get()` syntax
- ⚡ **Sync & Async Support**: Use blocking or async Redis connections (with feature flags)
- 🔧 **Extensible**: Easy to add custom commands or modify behavior

## Installation

```toml
[dependencies]
rhai-redis = "0.1"
```

## Quick Start

```rust
use rhai_redis::{RedisEngine, RedisClient};
use redis::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Redis
    let client = Client::open("redis://localhost:6379")?;
    let conn = client.get_connection()?;

    // Create engine and set Redis client
    let mut engine = RedisEngine::new();
    engine.set_redis_client(RedisClient::new(conn));

    // Run a script
    engine.run(r#"
        redis.set("key", "value");
        let value = redis.get("key");
        print("Got: " + value);
    "#)?;

    Ok(())
}
```

## API Reference

### String Operations
```rhai
redis.set("key", "value")
redis.get("key")
redis.del("key")
redis.exists("key")
redis.incr("counter")
redis.decr("counter")
```

### List Operations
```rhai
redis.lpush("list", "item")
redis.rpush("list", "item")
redis.lpop("list")
redis.rpop("list")
redis.lrange("list", 0, -1)
```

### Hash Operations
```rhai
redis.hset("hash", "field", "value")
redis.hget("hash", "field")
redis.hgetall("hash")
redis.hdel("hash", "field")
```

### Set Operations
```rhai
redis.sadd("set", "member")
redis.srem("set", "member")
redis.smembers("set")
redis.sismember("set", "member")
```

### Transactions
```rhai
redis.multi()
redis.set("key1", "value1")
redis.set("key2", "value2")
let results = redis.exec()
```

### Utility Functions
```rhai
print("output")
sleep(1000)  // milliseconds
let ts = timestamp()
let r = rand()  // 0.0 to 1.0
let n = rand_int(1, 100)
```

## Advanced Usage

### Running Scripts with Variables

```rust
engine.run_with_variables(
    r#"
        print("User ID: " + user_id);
        redis.hset("user:" + user_id, "last_seen", timestamp().to_string());
    "#,
    vec![("user_id".to_string(), "12345".to_string())]
)?;
```

### Custom Engine Configuration

```rust
let mut engine = RedisEngine::new();

// Access the underlying Rhai engine for customization
engine.engine()
    .set_max_operations(100_000)
    .set_max_expr_depths(50, 50);
```

## Feature Flags

- `default`: Includes synchronous support and utility functions
- `async`: Enable async/await support with Tokio
- `utils`: Include utility functions (rand, sleep, etc.)

## Safety & Security

Scripts run in a sandboxed environment with:
- Configurable recursion depth limits
- Maximum operation count limits
- No file system access
- No network access (except Redis)
- No system command execution

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
