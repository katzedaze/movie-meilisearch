use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::book::Book;
use crate::model::movie::Movie;
use crate::model::search::{FacetInfo, SearchResponse};
#[cfg(feature = "ssr")]
use crate::model::search::{FacetValue, SearchHit};

#[cfg(feature = "ssr")]
const HITS_PER_PAGE: usize = 12;

#[server]
pub async fn search_items(
    query: String,
    index: String,
    genres: Option<Vec<String>>,
    year_min: Option<i32>,
    year_max: Option<i32>,
    rating_min: Option<f64>,
    page: Option<usize>,
    sort: Option<String>,
) -> Result<SearchResponse, ServerFnError> {
    use crate::server::meilisearch::{build_filter, get_client};

    let genres = genres.unwrap_or_default();
    let client = get_client();
    let ms_index = client.index(&index);
    let current_page = page.unwrap_or(1);
    let offset = (current_page - 1) * HITS_PER_PAGE;

    let filter = build_filter(&genres, year_min, year_max, rating_min);

    let mut search = ms_index.search();
    search.with_query(&query);
    search.with_limit(HITS_PER_PAGE);
    search.with_offset(offset);

    if let Some(ref f) = filter {
        search.with_filter(f);
    }

    let sort_vec: Vec<String> = sort.into_iter().collect();
    let sort_refs: Vec<&str> = sort_vec.iter().map(|s| s.as_str()).collect();
    if !sort_refs.is_empty() {
        search.with_sort(&sort_refs);
    }

    search.with_show_ranking_score(true);

    let is_movie = index == "movies";

    if is_movie {
        let results = search
            .execute::<Movie>()
            .await
            .map_err(|e| ServerFnError::new(format!("Search failed: {e}")))?;

        let total_hits = results.estimated_total_hits.unwrap_or(0);
        let total_pages = (total_hits + HITS_PER_PAGE - 1) / HITS_PER_PAGE;

        let hits: Vec<SearchHit> = results
            .hits
            .into_iter()
            .map(|h| {
                let m = h.result;
                SearchHit {
                    id: m.id,
                    title: m.title,
                    title_en: m.title_en,
                    description: m.description,
                    creator: m.director,
                    year: m.year,
                    genres: m.genres,
                    rating: m.rating,
                    image_url: m.poster_url,
                    language: m.language,
                    index: "movies".to_string(),
                }
            })
            .collect();

        Ok(SearchResponse {
            hits,
            total_hits,
            page: current_page,
            total_pages,
            processing_time_ms: results.processing_time_ms,
        })
    } else {
        let results = search
            .execute::<Book>()
            .await
            .map_err(|e| ServerFnError::new(format!("Search failed: {e}")))?;

        let total_hits = results.estimated_total_hits.unwrap_or(0);
        let total_pages = (total_hits + HITS_PER_PAGE - 1) / HITS_PER_PAGE;

        let hits: Vec<SearchHit> = results
            .hits
            .into_iter()
            .map(|h| {
                let b = h.result;
                SearchHit {
                    id: b.id,
                    title: b.title,
                    title_en: b.title_en,
                    description: b.description,
                    creator: b.author,
                    year: b.year,
                    genres: b.genres,
                    rating: b.rating,
                    image_url: b.cover_url,
                    language: b.language,
                    index: "books".to_string(),
                }
            })
            .collect();

        Ok(SearchResponse {
            hits,
            total_hits,
            page: current_page,
            total_pages,
            processing_time_ms: results.processing_time_ms,
        })
    }
}

#[server]
pub async fn get_movie(id: i64) -> Result<Movie, ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("movies");
    let movie: Movie = index
        .get_document(&id.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Movie not found: {e}")))?;
    Ok(movie)
}

#[server]
pub async fn get_book(id: i64) -> Result<Book, ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("books");
    let book: Book = index
        .get_document(&id.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Book not found: {e}")))?;
    Ok(book)
}

#[server]
pub async fn create_movie(movie: Movie) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("movies");
    index
        .add_documents(&[movie], Some("id"))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create movie: {e}")))?;
    Ok(())
}

#[server]
pub async fn create_book(book: Book) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("books");
    index
        .add_documents(&[book], Some("id"))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create book: {e}")))?;
    Ok(())
}

#[server]
pub async fn update_movie(movie: Movie) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("movies");
    index
        .add_documents(&[movie], Some("id"))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update movie: {e}")))?;
    Ok(())
}

#[server]
pub async fn update_book(book: Book) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("books");
    index
        .add_documents(&[book], Some("id"))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update book: {e}")))?;
    Ok(())
}

#[server]
pub async fn delete_movie(id: i64) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("movies");
    index
        .delete_document(&id.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete movie: {e}")))?;
    Ok(())
}

#[server]
pub async fn delete_book(id: i64) -> Result<(), ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let index = client.index("books");
    index
        .delete_document(&id.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete book: {e}")))?;
    Ok(())
}

#[server]
pub async fn seed_data() -> Result<SeedResult, ServerFnError> {
    use crate::server::seed;

    let movies_count = seed::seed_movies()
        .await
        .map_err(|e| ServerFnError::new(e))?;
    let books_count = seed::seed_books()
        .await
        .map_err(|e| ServerFnError::new(e))?;

    Ok(SeedResult {
        movies_count,
        books_count,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedResult {
    pub movies_count: usize,
    pub books_count: usize,
}

#[server]
pub async fn get_facets(index: String) -> Result<FacetInfo, ServerFnError> {
    use crate::server::meilisearch::get_client;

    let client = get_client();
    let ms_index = client.index(&index);

    let mut search = ms_index.search();
    search.with_query("");
    search.with_limit(0);
    search.with_facets(meilisearch_sdk::search::Selectors::Some(&[
        "genres",
        "year",
        "language",
    ]));

    if index == "movies" {
        let results = search
            .execute::<Movie>()
            .await
            .map_err(|e| ServerFnError::new(format!("Facet query failed: {e}")))?;

        Ok(parse_facets(results.facet_distribution))
    } else {
        let results = search
            .execute::<Book>()
            .await
            .map_err(|e| ServerFnError::new(format!("Facet query failed: {e}")))?;

        Ok(parse_facets(results.facet_distribution))
    }
}

#[cfg(feature = "ssr")]
fn parse_facets(
    facet_distribution: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, usize>>,
    >,
) -> FacetInfo {
    let mut info = FacetInfo::default();

    if let Some(dist) = facet_distribution {
        if let Some(genres) = dist.get("genres") {
            info.genres = genres
                .iter()
                .map(|(k, v)| FacetValue {
                    value: k.clone(),
                    count: *v,
                })
                .collect();
        }
        if let Some(years) = dist.get("year") {
            info.years = years
                .iter()
                .map(|(k, v)| FacetValue {
                    value: k.clone(),
                    count: *v,
                })
                .collect();
        }
        if let Some(langs) = dist.get("language") {
            info.languages = langs
                .iter()
                .map(|(k, v)| FacetValue {
                    value: k.clone(),
                    count: *v,
                })
                .collect();
        }
    }

    info
}
