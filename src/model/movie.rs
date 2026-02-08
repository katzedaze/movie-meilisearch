use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    #[serde(default)]
    pub title_en: Option<String>,
    pub description: String,
    pub director: String,
    pub year: i32,
    pub genres: Vec<String>,
    pub rating: f64,
    #[serde(default)]
    pub poster_url: Option<String>,
    pub language: String,
}
