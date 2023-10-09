extern crate reqwest;
use core::result::Result;
use std::env;

mod db;
mod document_list;
mod get_documents;
mod models;
pub mod schema;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: get_documents::Config = envy::from_env()?;
    let docs = get_documents::get_documents(&config.readwise_api_key)?;
    // println!("{:#?}", docs);
    let first_doc = &docs[0];
    println!("{}", serde_json::to_string_pretty(first_doc)?);
    let conn = establish_connection();
    match db::create_result(&conn, &first_doc) {
        Ok(_) => println!("Successfully inserted result."),
        Err(e) => println!("Error inserting result: {:?}", e),
    }
    Ok(())
}
