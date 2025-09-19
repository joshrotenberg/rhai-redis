//! Redis Geo operations for Rhai integration

use crate::client::RedisClient;
use rhai::{Dynamic, Engine};

impl RedisClient {
    /// Add geospatial items
    pub fn geoadd(&mut self, key: &str, items: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(items);
        self.cmd("GEOADD", args)
    }

    /// Get distance between two members
    pub fn geodist(&mut self, key: &str, member1: &str, member2: &str, unit: &str) -> Dynamic {
        self.cmd(
            "GEODIST",
            vec![
                Dynamic::from(key.to_string()),
                Dynamic::from(member1.to_string()),
                Dynamic::from(member2.to_string()),
                Dynamic::from(unit.to_string()),
            ],
        )
    }

    /// Get geohash of members
    pub fn geohash(&mut self, key: &str, members: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(members);
        self.cmd("GEOHASH", args)
    }

    /// Get positions of members
    pub fn geopos(&mut self, key: &str, members: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(members);
        self.cmd("GEOPOS", args)
    }

    /// Search within radius
    pub fn georadius(&mut self, key: &str, longitude: f64, latitude: f64, radius: f64, unit: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(longitude),
            Dynamic::from(latitude),
            Dynamic::from(radius),
            Dynamic::from(unit.to_string()),
        ];
        args.extend(options);
        self.cmd("GEORADIUS", args)
    }

    /// Search within radius by member
    pub fn georadiusbymember(&mut self, key: &str, member: &str, radius: f64, unit: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(key.to_string()),
            Dynamic::from(member.to_string()),
            Dynamic::from(radius),
            Dynamic::from(unit.to_string()),
        ];
        args.extend(options);
        self.cmd("GEORADIUSBYMEMBER", args)
    }

    /// Search within box (Redis 6.2+)
    pub fn geosearch(&mut self, key: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![Dynamic::from(key.to_string())];
        args.extend(options);
        self.cmd("GEOSEARCH", args)
    }

    /// Store search results (Redis 6.2+)
    pub fn geosearchstore(&mut self, destination: &str, source: &str, options: Vec<Dynamic>) -> Dynamic {
        let mut args = vec![
            Dynamic::from(destination.to_string()),
            Dynamic::from(source.to_string()),
        ];
        args.extend(options);
        self.cmd("GEOSEARCHSTORE", args)
    }
}

/// Register Geo methods with Rhai engine
pub fn register_geo_methods(engine: &mut Engine) {
    engine
        .register_fn("geoadd", |client: &mut RedisClient, key: &str, items: Vec<Dynamic>| {
            client.geoadd(key, items)
        })
        .register_fn("geodist", |client: &mut RedisClient, key: &str, member1: &str, member2: &str, unit: &str| {
            client.geodist(key, member1, member2, unit)
        })
        .register_fn("geohash", |client: &mut RedisClient, key: &str, members: Vec<Dynamic>| {
            client.geohash(key, members)
        })
        .register_fn("geopos", |client: &mut RedisClient, key: &str, members: Vec<Dynamic>| {
            client.geopos(key, members)
        })
        .register_fn("georadius", |client: &mut RedisClient, key: &str, longitude: f64, latitude: f64, radius: f64, unit: &str, options: Vec<Dynamic>| {
            client.georadius(key, longitude, latitude, radius, unit, options)
        })
        .register_fn("georadiusbymember", |client: &mut RedisClient, key: &str, member: &str, radius: f64, unit: &str, options: Vec<Dynamic>| {
            client.georadiusbymember(key, member, radius, unit, options)
        })
        .register_fn("geosearch", |client: &mut RedisClient, key: &str, options: Vec<Dynamic>| {
            client.geosearch(key, options)
        })
        .register_fn("geosearchstore", |client: &mut RedisClient, destination: &str, source: &str, options: Vec<Dynamic>| {
            client.geosearchstore(destination, source, options)
        });
}
