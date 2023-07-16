extern crate reqwest;
use std::env;
use std::{thread, time};

use reqwest::header;

mod document_list;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let readwise_api_key = env::var("READWISE_API_KEY").ok().unwrap();
    let mut headers = header::HeaderMap::new();
    let mut cursor = "".to_string();
    let mut done = false;
    let mut docs = Vec::<document_list::Result>::new();
    while !done {
        headers.insert(
            "Authorization",
            format!("Token {}", readwise_api_key).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .connection_verbose(true)
            .build()
            .unwrap();
        let query = if cursor.is_empty() {
            vec![("location", "feed")]
        } else {
            vec![("location", "feed"), ("pageCursor", cursor.as_str())]
        };
        let builder = client
            .get("https://readwise.io/api/v3/list/")
            .query(query.as_slice());

        let builder2 = client
            .get("https://readwise.io/api/v3/list/")
            .query(query.as_slice());
        let txt = builder2.headers(headers.clone()).send()?.text()?;
        println!("{}", txt);
        let result: document_list::Root = builder.headers(headers.clone()).send()?.json()?;
        docs.extend(result.results);
        cursor = match result.next_page_cursor {
            Some(c) => c,
            None => "".to_string(),
        };
        println!("cursor: {},count: {}", cursor, docs.len());
        if cursor.is_empty() {
            done = true
        }
        // TODO: properly handle rate limits
        thread::sleep(time::Duration::from_secs(10));
        println!("=================================")
    }
    println!("Num docs: {}", docs.len());
    // headers.insert(
    //     "Authorization",
    //     format!("Token {}", readwise_api_key).parse().unwrap(),
    // );
    // headers.insert("Content-Type", "application/json".parse().unwrap());
    // let client = reqwest::blocking::Client::builder()
    //     .redirect(reqwest::redirect::Policy::none())
    //     .build()
    //     .unwrap();
    // let res = client
    //     .get("https://readwise.io/api/v3/list/?location=feed")
    //     .headers(headers.clone())
    //     .send()?
    //     .text()?;
    // println!("{}", res);
    // let result: document_list::Root = client
    //     .get("https://readwise.io/api/v3/list/?location=feed")
    //     .headers(headers.clone())
    //     .send()?
    //     .json()?;
    // println!("{:#?}", result.next_page_cursor);

    Ok(())
}
