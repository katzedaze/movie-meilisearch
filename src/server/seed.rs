use crate::model::book::Book;
use crate::model::movie::Movie;
use crate::server::meilisearch::{configure_index, get_client};

pub async fn seed_movies() -> Result<usize, String> {
    let data: &str = include_str!("../../seed/movies.json");
    let movies: Vec<Movie> =
        serde_json::from_str(data).map_err(|e| format!("Failed to parse movies.json: {e}"))?;

    let count = movies.len();
    let client = get_client();
    let index = client.index("movies");

    let task = index
        .add_documents(&movies, Some("id"))
        .await
        .map_err(|e| format!("Failed to add movies: {e}"))?;

    task.wait_for_completion(client, None, None)
        .await
        .map_err(|e| format!("Failed waiting for movie indexing: {e}"))?;

    configure_index("movies", true).await?;

    Ok(count)
}

pub async fn seed_books() -> Result<usize, String> {
    let data: &str = include_str!("../../seed/books.json");
    let books: Vec<Book> =
        serde_json::from_str(data).map_err(|e| format!("Failed to parse books.json: {e}"))?;

    let count = books.len();
    let client = get_client();
    let index = client.index("books");

    let task = index
        .add_documents(&books, Some("id"))
        .await
        .map_err(|e| format!("Failed to add books: {e}"))?;

    task.wait_for_completion(client, None, None)
        .await
        .map_err(|e| format!("Failed waiting for book indexing: {e}"))?;

    configure_index("books", false).await?;

    Ok(count)
}
