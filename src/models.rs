use crate::schema::results;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::Value; // You need the `chrono` crate for date-time support

#[derive(Queryable, Insertable, Debug)]
#[table_name = "results"]
pub struct Result {
    pub id: String,
    pub url: String,
    pub source_url: String,
    pub title: String,
    pub author: String,
    pub source: Option<String>,
    pub category: String,
    pub location: String,
    pub tags: String,
    pub site_name: Option<String>,
    pub word_count: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub notes: String,
    pub published_date: String,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub parent_id: String,
    pub reading_progress: f64,
}
