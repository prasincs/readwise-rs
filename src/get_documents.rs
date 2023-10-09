use core::result::Result;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

use crate::document_list;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) readwise_api_key: String,
}

pub fn get_documents(
    readwise_api_key: &str,
) -> Result<Vec<document_list::Result>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let mut cursor = "".to_string();
    let mut done = false;
    let mut docs = Vec::<document_list::Result>::new();
    let mut tags = Vec::<&String>::new();
    while !done {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", readwise_api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .connection_verbose(true)
            .build()?;
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
        //println!("{}", txt);
        let result: document_list::Root = builder.headers(headers.clone()).send()?.json()?;
        docs.extend(result.results);
        if let Some(next_cursor) = result.next_page_cursor {
            cursor = next_cursor;
        } else {
            done = true;
        }
    }
    Ok(docs)
}
