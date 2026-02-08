use leptos::prelude::*;

use crate::components::result_card::ResultCard;
use crate::model::search::SearchResponse;

#[component]
pub fn SearchResults(
    results: Signal<Option<Result<SearchResponse, ServerFnError>>>,
    loading: Signal<bool>,
    query: Signal<String>,
    on_web_import: impl Fn() + 'static + Copy + Send,
    web_importing: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="search-results">
            <Suspense fallback=move || view! { <div class="loading">"読み込み中..."</div> }>
                {move || {
                    if loading.get() {
                        return view! { <div class="loading">"検索中..."</div> }.into_any();
                    }

                    match results.get() {
                        None => view! {
                            <div class="no-results">
                                <p>"検索キーワードを入力してください"</p>
                            </div>
                        }.into_any(),
                        Some(Ok(response)) => {
                            if response.hits.is_empty() {
                                let q = query.get();
                                let has_query = !q.is_empty();
                                view! {
                                    <div class="no-results">
                                        <p>"結果が見つかりませんでした"</p>
                                        {if has_query {
                                            Some(view! {
                                                <button
                                                    class="web-import-btn"
                                                    on:click=move |_| on_web_import()
                                                    disabled=web_importing
                                                >
                                                    {move || if web_importing.get() {
                                                        "Web検索中..."
                                                    } else {
                                                        "Web検索して取り込む"
                                                    }}
                                                </button>
                                            })
                                        } else {
                                            None
                                        }}
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div>
                                        <div class="results-meta">
                                            <span>{response.total_hits}" 件の結果"</span>
                                            <span class="processing-time">"("{response.processing_time_ms}" ms)"</span>
                                        </div>
                                        <div class="results-grid">
                                            {response.hits.into_iter().map(|hit| {
                                                view! { <ResultCard hit=hit/> }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        },
                        Some(Err(e)) => view! {
                            <div class="error">
                                <p>"エラーが発生しました: "{e.to_string()}</p>
                            </div>
                        }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}
