use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub count: i64,
    pub next_page_cursor: String,
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
    pub source: String,
    pub category: String,
    pub location: String,
    pub tags: Tags,
    #[serde(rename = "site_name")]
    pub site_name: String,
    #[serde(rename = "word_count")]
    pub word_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    pub notes: String,
    #[serde(rename = "published_date")]
    pub published_date: NaiveDate,
    pub summary: String,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    #[serde(rename = "reading_progress")]
    pub reading_progress: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {}
