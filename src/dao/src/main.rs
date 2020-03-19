#[macro_use]
extern crate diesel;

use serde_derive::Serialize;
use diesel::sql_query;
use diesel::prelude::*;
use std::error::Error;

mod pg;
mod cache;
mod schema;

use redis::Commands;
use schema::product;


#[derive(Debug, QueryableByName)]
#[table_name = "product"]
struct Product {

    id: i32,

    uid: i32,

    count: i32,
}


fn main() -> Result<(), Box<dyn Error>> {
    pg::init();
    cache::init();
    let conn = pg::connection()?;
    let mut red = cache::connection()?;
    let _: () = red.set_ex("aa", 1, 3600)?;
    let v: String = red.get("aa").unwrap();
    println!("{}", v);


    Ok(())
}
