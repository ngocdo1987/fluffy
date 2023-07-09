use redis::{Client, Connection};
use lazy_static::*;
use std::sync::{Mutex};

lazy_static! {
    static ref CACHES: Mutex<Vec<Client>> = Mutex::new(vec![]);
}

/// Initialization database connection
///
/// Connect string looks like: 'redis://127.0.0.1'
pub fn init_connections(conn_string: &str) { 
    println!("Create cache connection: {}", conn_string);
    let cache = redis::Client::open(conn_string).unwrap();
    let mut pools = CACHES.lock().unwrap();
    (*pools).push(cache);
}

/// Get the database connection
///
/// ```rust
/// let mut redis = cache::get_conn();
/// let _val = redis.get::<&str, String>("hello").unwrap_or("world".to_owned());
/// ```
pub fn get_conn() -> Connection { 
    let pools = CACHES.lock().unwrap();
    unsafe { 
        (*pools).get_unchecked(0).get_connection().unwrap()
    }
}
