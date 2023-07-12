extern crate reqwest;
use std::env;

use reqwest::header;

mod document_list;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let readwise_api_key = env::var("READWISE_API_KEY").ok().unwrap();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Token {}", readwise_api_key).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://readwise.io/api/v3/list/?location=feed")
        .headers(headers.clone())
        .send()?
        .text()?;
    println!("{}", res);
    let result: document_list::Root = client
        .get("https://readwise.io/api/v3/list/?location=feed")
        .headers(headers.clone())
        .send()?
        .json()?;
    println!("{:#?}", result);

    Ok(())
}
