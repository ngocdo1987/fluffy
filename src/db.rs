use std::ops::DerefMut;
use mysql::{self, Pool, PooledConn};
use lazy_static::*;
use std::sync::{Mutex};

use crate::Db;

lazy_static! {
    static ref POOLS: Mutex<Vec<Pool>> = Mutex::new(vec![]);
}

#[macro_export]
macro_rules! conn {
    ($pool: expr) => ({
        crate::db::get($pool)
    })
}

#[macro_export]
macro_rules! transaction {
    ($conn: expr) => ({
        $conn.start_transaction(false, None, None).unwrap()
    })
}

#[macro_export]
macro_rules! from_row {
    ($row: expr) => ({
        mysql::from_row($row)
    })
}

/// Initialization database connection
pub fn init_connections(conn_string: &str) { 
    println!("Create data connection: {}", conn_string);
    let pool = mysql::Pool::new(conn_string).unwrap();
    let mut pools = POOLS.lock().unwrap();
    (*pools).push(pool);
}

/// Get the database connection
pub fn get_conn() -> PooledConn { 
    let pools = POOLS.lock().unwrap();
    unsafe { 
        (*pools).get_unchecked(0).get_conn().unwrap()
    }
}

/// deref the reference of pool
pub fn get(pool: &Db) -> PooledConn { 
    pool.lock().unwrap().deref_mut().get_conn().unwrap()
}
