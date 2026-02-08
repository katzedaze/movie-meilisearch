use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebResult {
    pub id: i64,
    pub title: String,
    #[serde(default)]
    pub title_en: Option<String>,
    pub description: String,
    pub url: String,
    #[serde(default)]
    pub source_engine: Option<String>,
    pub year: i32,
    #[serde(default)]
    pub genres: Vec<String>,
    pub rating: f64,
    #[serde(default)]
    pub image_url: Option<String>,
    pub language: String,
    #[serde(default)]
    pub published_date: Option<String>,
}
