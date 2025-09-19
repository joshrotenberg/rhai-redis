use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rhai_redis::{RedisEngine, RedisClient};
use redis::Client;

fn setup_engine() -> RedisEngine {
    let client = Client::open("redis://localhost:6379").expect("Failed to connect");
    let conn = client.get_connection().expect("Failed to get connection");
    let redis_client = RedisClient::new(conn);
    
    let mut engine = RedisEngine::new();
    engine.set_redis_client(redis_client);
    engine
}

fn benchmark_string_operations(c: &mut Criterion) {
    let mut engine = setup_engine();
    
    c.bench_function("set_get_100", |b| {
        b.iter(|| {
            engine.run(r#"
                for i in 0..100 {
                    redis.set("bench:key" + i.to_string(), "value" + i.to_string());
                    redis.get("bench:key" + i.to_string());
                }
            "#).unwrap();
        })
    });
}

fn benchmark_list_operations(c: &mut Criterion) {
    let mut engine = setup_engine();
    
    c.bench_function("lpush_lpop_100", |b| {
        engine.run("redis.del('bench:list')").unwrap();
        b.iter(|| {
            engine.run(r#"
                for i in 0..100 {
                    redis.lpush("bench:list", "item" + i.to_string());
                }
                for i in 0..100 {
                    redis.lpop("bench:list");
                }
            "#).unwrap();
        })
    });
}

fn benchmark_sorted_set_operations(c: &mut Criterion) {
    let mut engine = setup_engine();
    
    c.bench_function("zadd_zrank_100", |b| {
        engine.run("redis.del('bench:zset')").unwrap();
        b.iter(|| {
            engine.run(r#"
                for i in 0..100 {
                    redis.zadd("bench:zset", i.to_float(), "member" + i.to_string());
                }
            "#).unwrap();
        })
    });
}

criterion_group!(
    benches,
    benchmark_string_operations,
    benchmark_list_operations,
    benchmark_sorted_set_operations
);
criterion_main!(benches);
