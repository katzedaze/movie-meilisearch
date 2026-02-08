use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::signal_debounced;

use crate::api::{get_facets, search_items, seed_data};
use crate::components::facet_panel::FacetPanel;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::components::search_results::SearchResults;
use crate::model::search::{FacetInfo, SearchResponse};

#[component]
pub fn HomePage() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (index, set_index) = signal("movies".to_string());
    let (selected_genres, set_selected_genres) = signal(Vec::<String>::new());
    let (year_min, set_year_min) = signal(Option::<i32>::None);
    let (year_max, set_year_max) = signal(Option::<i32>::None);
    let (rating_min, set_rating_min) = signal(Option::<f64>::None);
    let (page, set_page) = signal(1usize);
    let (sort, set_sort) = signal(Option::<String>::None);
    let (seeding, set_seeding) = signal(false);
    let (seed_message, set_seed_message) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let debounced_query: Signal<String> = signal_debounced(query, 300.0);

    // Search results
    let (results, set_results) = signal(Option::<Result<SearchResponse, ServerFnError>>::None);

    // Facets
    let (facets, set_facets) = signal(Option::<Result<FacetInfo, ServerFnError>>::None);

    // Fetch facets when index changes
    Effect::new(move |_| {
        let idx = index.get();
        spawn_local(async move {
            let result = get_facets(idx).await;
            set_facets.set(Some(result));
        });
    });

    // Search when debounced query or filters change
    Effect::new(move |_| {
        let q = debounced_query.get();
        let idx = index.get();
        let genres = selected_genres.get();
        let y_min = year_min.get();
        let y_max = year_max.get();
        let r_min = rating_min.get();
        let p = page.get();
        let s = sort.get();

        if q.is_empty() && genres.is_empty() && y_min.is_none() && y_max.is_none() && r_min.is_none() {
            set_results.set(None);
            return;
        }

        set_loading.set(true);
        spawn_local(async move {
            let result = search_items(q, idx, Some(genres), y_min, y_max, r_min, Some(p), s).await;
            set_results.set(Some(result));
            set_loading.set(false);
        });
    });

    // Reset page to 1 when filters change
    Effect::new(move |_| {
        let _ = debounced_query.get();
        let _ = selected_genres.get();
        let _ = year_min.get();
        let _ = year_max.get();
        let _ = rating_min.get();
        let _ = sort.get();
        set_page.set(1);
    });

    let on_seed = move || {
        set_seeding.set(true);
        set_seed_message.set(None);
        spawn_local(async move {
            match seed_data().await {
                Ok(result) => {
                    set_seed_message.set(Some(format!(
                        "投入完了: 映画 {} 件, 書籍 {} 件",
                        result.movies_count, result.books_count
                    )));
                    // Refresh facets
                    let idx = index.get_untracked();
                    if let Ok(facet_info) = get_facets(idx).await {
                        set_facets.set(Some(Ok(facet_info)));
                    }
                }
                Err(e) => {
                    set_seed_message.set(Some(format!("エラー: {e}")));
                }
            }
            set_seeding.set(false);
        });
    };

    let total_pages = Signal::derive(move || {
        results
            .get()
            .and_then(|r| r.ok())
            .map(|r| r.total_pages)
            .unwrap_or(0)
    });

    let current_page = Signal::derive(move || page.get());

    let on_page_change = move |new_page: usize| {
        set_page.set(new_page);
    };

    view! {
        <div class="home-page">
            <SearchBar
                query=query
                set_query=set_query
                index=index
                set_index=set_index
                on_seed=on_seed
                seeding=seeding
            />

            {move || seed_message.get().map(|msg| view! {
                <div class="seed-message">{msg}</div>
            })}

            <div class="content-layout">
                <FacetPanel
                    facets=facets.into()
                    selected_genres=selected_genres
                    set_selected_genres=set_selected_genres
                    year_min=year_min
                    set_year_min=set_year_min
                    year_max=year_max
                    set_year_max=set_year_max
                    rating_min=rating_min
                    set_rating_min=set_rating_min
                    set_sort=set_sort
                />
                <div class="results-section">
                    <SearchResults
                        results=results.into()
                        loading=loading.into()
                    />
                    <Pagination
                        current_page=current_page
                        total_pages=total_pages
                        on_page_change=on_page_change
                    />
                </div>
            </div>
        </div>
    }
}
