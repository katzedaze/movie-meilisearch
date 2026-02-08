use leptos::prelude::*;

use crate::model::search::SearchHit;

#[component]
pub fn ResultCard(hit: SearchHit) -> impl IntoView {
    let detail_url = if hit.index == "movies" {
        format!("/movie/{}", hit.id)
    } else {
        format!("/book/{}", hit.id)
    };

    let type_label = if hit.index == "movies" {
        "映画"
    } else {
        "書籍"
    };

    let rating_display = format!("{:.1}", hit.rating);
    let creator_year = format!("{} ({})", hit.creator, hit.year);
    let description_truncated = if hit.description.chars().count() > 120 {
        let truncated: String = hit.description.chars().take(120).collect();
        format!("{truncated}...")
    } else {
        hit.description.clone()
    };

    view! {
        <a href=detail_url class="result-card">
            <div class="card-header">
                <span class="card-type">{type_label}</span>
                <span class="card-rating">"★ "{rating_display}</span>
            </div>
            <h3 class="card-title">{hit.title.clone()}</h3>
            {hit.title_en.map(|en| view! {
                <p class="card-title-en">{en}</p>
            })}
            <p class="card-creator">{creator_year}</p>
            <p class="card-description">{description_truncated}</p>
            <div class="card-genres">
                {hit.genres.into_iter().map(|g| view! {
                    <span class="genre-tag">{g}</span>
                }).collect_view()}
            </div>
        </a>
    }
}
