use std::ops::DerefMut;
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::info;
use crate::dep::config::CONFIG;


lazy_static! {
    pub static ref REDIS: Mutex<redis::Connection> = {
        info!("Redis initialized");
        let redis_url = CONFIG.get_string("app.redis.url").unwrap();
        let redis_client = redis::Client::open(redis_url).unwrap();
        Mutex::new(redis_client.get_connection().unwrap())
    };
}

pub fn increment(key: &str) {
    redis::cmd("INCR").arg(key).query::<usize>(REDIS.lock().unwrap().deref_mut()).unwrap();
}
