use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::model::search::FacetInfo;

#[component]
pub fn FacetPanel(
    facets: Signal<Option<Result<FacetInfo, ServerFnError>>>,
    selected_genres: ReadSignal<Vec<String>>,
    set_selected_genres: WriteSignal<Vec<String>>,
    year_min: ReadSignal<Option<i32>>,
    set_year_min: WriteSignal<Option<i32>>,
    year_max: ReadSignal<Option<i32>>,
    set_year_max: WriteSignal<Option<i32>>,
    rating_min: ReadSignal<Option<f64>>,
    set_rating_min: WriteSignal<Option<f64>>,
    set_sort: WriteSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <aside class="facet-panel">
            <h3 class="facet-title">"フィルタ"</h3>

            // Sort
            <div class="facet-section">
                <h4>"並び替え"</h4>
                <select
                    class="sort-select"
                    on:change=move |ev| {
                        let target = ev.target().unwrap();
                        let select = target.unchecked_ref::<web_sys::HtmlInputElement>();
                        let val = select.value();
                        if val.is_empty() {
                            set_sort.set(None);
                        } else {
                            set_sort.set(Some(val));
                        }
                    }
                >
                    <option value="">"関連度順"</option>
                    <option value="year:desc">"年 (新しい順)"</option>
                    <option value="year:asc">"年 (古い順)"</option>
                    <option value="rating:desc">"評価 (高い順)"</option>
                    <option value="rating:asc">"評価 (低い順)"</option>
                    <option value="title:asc">"タイトル (A→Z)"</option>
                </select>
            </div>

            // Year range
            <div class="facet-section">
                <h4>"年範囲"</h4>
                <div class="range-inputs">
                    <input
                        type="number"
                        class="range-input"
                        placeholder="最小"
                        prop:value=move || year_min.get().map(|v| v.to_string()).unwrap_or_default()
                        on:change=move |ev| {
                            let target = ev.target().unwrap();
                            let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
                            let val = input.value();
                            set_year_min.set(val.parse().ok());
                        }
                    />
                    <span class="range-sep">"〜"</span>
                    <input
                        type="number"
                        class="range-input"
                        placeholder="最大"
                        prop:value=move || year_max.get().map(|v| v.to_string()).unwrap_or_default()
                        on:change=move |ev| {
                            let target = ev.target().unwrap();
                            let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
                            let val = input.value();
                            set_year_max.set(val.parse().ok());
                        }
                    />
                </div>
            </div>

            // Rating minimum
            <div class="facet-section">
                <h4>"最低評価"</h4>
                <div class="rating-filter">
                    <input
                        type="range"
                        min="0"
                        max="10"
                        step="0.5"
                        class="rating-slider"
                        prop:value=move || rating_min.get().map(|v| v.to_string()).unwrap_or_else(|| "0".to_string())
                        on:input=move |ev| {
                            let target = ev.target().unwrap();
                            let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
                            let val: f64 = input.value().parse().unwrap_or(0.0);
                            if val > 0.0 {
                                set_rating_min.set(Some(val));
                            } else {
                                set_rating_min.set(None);
                            }
                        }
                    />
                    <span class="rating-value">
                        {move || {
                            rating_min.get()
                                .map(|v| format!("★ {v:.1}+"))
                                .unwrap_or_else(|| "すべて".to_string())
                        }}
                    </span>
                </div>
            </div>

            // Genres
            <div class="facet-section">
                <h4>"ジャンル"</h4>
                {move || {
                    match facets.get() {
                        Some(Ok(info)) => {
                            view! {
                                <div class="genre-checkboxes">
                                    {info.genres.into_iter().map(|fv| {
                                        let genre = fv.value.clone();
                                        let genre_for_check = genre.clone();
                                        let genre_for_change = genre.clone();
                                        let count = fv.count;
                                        view! {
                                            <label class="genre-checkbox">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || selected_genres.get().contains(&genre_for_check)
                                                    on:change=move |_| {
                                                        let mut current = selected_genres.get();
                                                        if current.contains(&genre_for_change) {
                                                            current.retain(|g| g != &genre_for_change);
                                                        } else {
                                                            current.push(genre_for_change.clone());
                                                        }
                                                        set_selected_genres.set(current);
                                                    }
                                                />
                                                <span>{format!("{genre} ({count})")}</span>
                                            </label>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        },
                        _ => view! { <p class="facet-loading">"読み込み中..."</p> }.into_any(),
                    }
                }}
            </div>

            // Clear filters
            <button
                class="clear-filters"
                on:click=move |_| {
                    set_selected_genres.set(vec![]);
                    set_year_min.set(None);
                    set_year_max.set(None);
                    set_rating_min.set(None);
                    set_sort.set(None);
                }
            >
                "フィルタをクリア"
            </button>
        </aside>
    }
}
