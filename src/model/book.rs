use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub id: i64,
    pub title: String,
    #[serde(default)]
    pub title_en: Option<String>,
    pub description: String,
    pub author: String,
    pub year: i32,
    pub genres: Vec<String>,
    pub rating: f64,
    #[serde(default)]
    pub cover_url: Option<String>,
    pub language: String,
    #[serde(default)]
    pub pages: Option<i32>,
}
