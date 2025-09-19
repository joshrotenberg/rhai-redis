//! Utility functions for Rhai scripting

use rhai::Engine;

pub fn register_utility_functions(engine: &mut Engine) {
    // Sleep function for delays (milliseconds)
    engine.register_fn("sleep", |millis: i64| {
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
    });

    // Time function
    engine.register_fn("timestamp", || -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });

    // Random number generation
    engine.register_fn("rand", || -> f64 {
        use rand::Rng;
        rand::thread_rng().r#gen::<f64>()
    });

    engine.register_fn("rand_int", |min: i64, max: i64| -> i64 {
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
    });
}
