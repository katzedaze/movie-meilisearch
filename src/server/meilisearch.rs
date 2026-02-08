use meilisearch_sdk::client::Client;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        let url = std::env::var("MEILI_URL").unwrap_or_else(|_| "http://localhost:7700".to_string());
        let key = std::env::var("MEILI_MASTER_KEY").unwrap_or_else(|_| "masterKey".to_string());
        Client::new(url, Some(key)).expect("Failed to create Meilisearch client")
    })
}

pub async fn configure_index(index_name: &str, is_movie: bool) -> Result<(), String> {
    let client = get_client();
    let index = client.index(index_name);

    let searchable = if is_movie {
        vec!["title", "title_en", "description", "director", "genres"]
    } else {
        vec!["title", "title_en", "description", "author", "genres"]
    };

    let filterable = vec!["genres", "year", "rating", "language"];
    let sortable = vec!["year", "rating", "title"];

    index
        .set_searchable_attributes(&searchable)
        .await
        .map_err(|e| format!("Failed to set searchable attributes: {e}"))?;

    index
        .set_filterable_attributes(&filterable)
        .await
        .map_err(|e| format!("Failed to set filterable attributes: {e}"))?;

    index
        .set_sortable_attributes(&sortable)
        .await
        .map_err(|e| format!("Failed to set sortable attributes: {e}"))?;

    Ok(())
}

pub async fn configure_web_index() -> Result<(), String> {
    let client = get_client();
    let index = client.index("web");

    let searchable = vec!["title", "description", "url", "source_engine"];
    let filterable = vec!["genres", "year", "rating", "language"];
    let sortable = vec!["year", "rating", "title"];

    index
        .set_searchable_attributes(&searchable)
        .await
        .map_err(|e| format!("Failed to set searchable attributes: {e}"))?;

    index
        .set_filterable_attributes(&filterable)
        .await
        .map_err(|e| format!("Failed to set filterable attributes: {e}"))?;

    index
        .set_sortable_attributes(&sortable)
        .await
        .map_err(|e| format!("Failed to set sortable attributes: {e}"))?;

    Ok(())
}

pub fn build_filter(
    genres: &[String],
    year_min: Option<i32>,
    year_max: Option<i32>,
    rating_min: Option<f64>,
) -> Option<String> {
    let mut conditions = Vec::new();

    if !genres.is_empty() {
        let genre_filters: Vec<String> = genres
            .iter()
            .map(|g| format!("genres = \"{}\"", g.replace('"', "\\\"")))
            .collect();
        conditions.push(format!("({})", genre_filters.join(" OR ")));
    }

    if let Some(min) = year_min {
        conditions.push(format!("year >= {min}"));
    }

    if let Some(max) = year_max {
        conditions.push(format!("year <= {max}"));
    }

    if let Some(min) = rating_min {
        conditions.push(format!("rating >= {min}"));
    }

    if conditions.is_empty() {
        None
    } else {
        Some(conditions.join(" AND "))
    }
}
