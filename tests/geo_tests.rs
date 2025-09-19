#[cfg(test)]
mod geo_tests {
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
    #[ignore]
    fn test_geo_basic() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add geo locations (longitude, latitude, member)
            let added = redis.geoadd("cities", [
                -122.419, 37.775, "San Francisco",
                -74.006, 40.713, "New York",
                -0.128, 51.507, "London"
            ]);
            print("Added " + added.to_string() + " locations");
            
            // Get distance between cities
            let distance_km = redis.geodist("cities", "San Francisco", "New York", "km");
            print("SF to NY: " + distance_km.to_string() + " km");
            
            let distance_mi = redis.geodist("cities", "San Francisco", "New York", "mi");
            print("SF to NY: " + distance_mi.to_string() + " miles");
        "#;
        
        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_geo_hash_and_pos() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add locations
            redis.geoadd("cities", [
                -122.419, 37.775, "San Francisco",
                -74.006, 40.713, "New York"
            ]);
            
            // Get geohash
            let hashes = redis.geohash("cities", ["San Francisco", "New York"]);
            print("Geohashes: " + hashes.to_string());
            
            // Get positions
            let positions = redis.geopos("cities", ["San Francisco", "New York"]);
            print("Positions: " + positions.to_string());
        "#;
        
        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_geo_radius_search() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add several locations
            redis.geoadd("stores", [
                -122.419, 37.775, "Store A",
                -122.420, 37.776, "Store B",
                -122.421, 37.777, "Store C",
                -122.430, 37.780, "Store D",
                -122.440, 37.790, "Store E"
            ]);
            
            // Find stores within 1km of a point
            let nearby = redis.georadius("stores", -122.420, 37.775, 1.0, "km", []);
            print("Stores within 1km: " + nearby.to_string());
            
            // With distance
            let with_dist = redis.georadius("stores", -122.420, 37.775, 2.0, "km", 
                ["WITHDIST", "SORT", "ASC"]);
            print("Stores with distance: " + with_dist.to_string());
            
            // Find stores near Store A
            let near_store = redis.georadiusbymember("stores", "Store A", 1.0, "km", 
                ["WITHDIST"]);
            print("Near Store A: " + near_store.to_string());
        "#;
        
        engine.run(script).expect("Script failed");
    }

    #[test]
    #[ignore]
    fn test_geo_search() {
        let mut engine = setup();
        
        let script = r#"
            redis.cmd("FLUSHDB", []);
            
            // Add locations
            redis.geoadd("locations", [
                -122.419, 37.775, "Point A",
                -122.420, 37.776, "Point B",
                -122.421, 37.777, "Point C"
            ]);
            
            // Search within box (Redis 6.2+)
            // Note: This might fail on older Redis versions
            let results = redis.geosearch("locations", [
                "FROMLONLAT", -122.420, 37.775,
                "BYRADIUS", 2, "km",
                "ASC"
            ]);
            print("Search results: " + results.to_string());
        "#;
        
        engine.run(script).expect("Script failed");
    }
}
