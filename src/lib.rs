use actix_web::{web};
use mysql::{Pool};
use std::sync::{Mutex};
use serde_derive::{Deserialize, Serialize};

pub type Db = web::Data<Mutex<Pool>>;
pub type DbRow = mysql::Row;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pager { 
    pub pages: u32,
    pub current: u32,
    pub rows_total: u32,
    pub limit: u32,
}

pub mod math;
pub mod random;
pub mod numbers;
#[macro_use]
pub mod db;
pub mod cache;
pub mod utils;
pub mod datetime;
pub mod validation;
//pub mod jwt;
pub mod cors;
#[macro_use]
pub mod tmpl;
#[macro_use]
pub mod router;
#[macro_use]
pub mod cond_builder;
#[macro_use]
pub mod data_set;
pub mod model;
#[macro_use]
pub mod query_builder;
pub mod request;
pub mod response;
pub mod constants;
//pub mod middlewares;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let query = query![
            fields => "id, name",
        ];
        println!("query = {:?}", query);
        assert_eq!(2 + 2, 4);
    }
}
