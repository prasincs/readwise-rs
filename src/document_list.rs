use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub count: i64,
    pub next_page_cursor: Option<String>,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: String,
    pub url: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    pub title: String,
    pub author: String,
    pub source: Option<String>,
    pub category: String,
    pub location: String,
    pub tags: HashMap<String, serde_json::Value>,
    #[serde(rename = "site_name")]
    pub site_name: Option<String>,
    #[serde(rename = "word_count")]
    pub word_count: Option<i64>,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    pub notes: String,
    #[serde(rename = "published_date")]
    // published_date can sometimes be unix time so cannot use NaiveDate
    pub published_date: Value,
    pub summary: Option<String>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    #[serde(rename = "reading_progress")]
    pub reading_progress: f64,
}
