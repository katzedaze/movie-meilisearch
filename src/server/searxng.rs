use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::OnceLock;

use reqwest::Client;
use serde::Deserialize;

use crate::model::web_result::WebResult;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(Client::new)
}

fn get_searxng_url() -> String {
    std::env::var("SEARXNG_URL").unwrap_or_else(|_| "http://searxng:8080".to_string())
}

#[derive(Debug, Deserialize)]
struct SearxngResponse {
    results: Vec<SearxngResult>,
}

#[derive(Debug, Deserialize)]
struct SearxngResult {
    title: String,
    url: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    engine: Option<String>,
    #[serde(default, rename = "publishedDate")]
    published_date: Option<String>,
    #[serde(default)]
    img_src: Option<String>,
}

fn url_to_id(url: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    (hasher.finish() as i64).abs()
}

pub async fn search_web(query: &str) -> Result<Vec<WebResult>, String> {
    let base_url = get_searxng_url();
    let client = get_http_client();

    let resp = client
        .get(format!("{base_url}/search"))
        .query(&[("q", query), ("format", "json")])
        .send()
        .await
        .map_err(|e| format!("SearXNG request failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("SearXNG returned status: {}", resp.status()));
    }

    let body: SearxngResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse SearXNG response: {e}"))?;

    let results = body
        .results
        .into_iter()
        .map(|r| WebResult {
            id: url_to_id(&r.url),
            title: r.title,
            title_en: None,
            description: r.content.unwrap_or_default(),
            url: r.url,
            source_engine: r.engine,
            year: 0,
            genres: vec!["web".to_string()],
            rating: 0.0,
            image_url: r.img_src,
            language: "web".to_string(),
            published_date: r.published_date,
        })
        .collect();

    Ok(results)
}
