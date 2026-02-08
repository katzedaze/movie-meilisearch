use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    pub query: String,
    pub index: String,
    #[serde(default)]
    pub genres: Vec<String>,
    pub year_min: Option<i32>,
    pub year_max: Option<i32>,
    pub rating_min: Option<f64>,
    pub page: Option<usize>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResponse {
    pub hits: Vec<SearchHit>,
    pub total_hits: usize,
    pub page: usize,
    pub total_pages: usize,
    pub processing_time_ms: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: i64,
    pub title: String,
    pub title_en: Option<String>,
    pub description: String,
    pub creator: String,
    pub year: i32,
    pub genres: Vec<String>,
    pub rating: f64,
    pub image_url: Option<String>,
    pub language: String,
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FacetInfo {
    pub genres: Vec<FacetValue>,
    pub years: Vec<FacetValue>,
    pub languages: Vec<FacetValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetValue {
    pub value: String,
    pub count: usize,
}
