use leptos::prelude::*;

#[component]
pub fn Pagination(
    current_page: Signal<usize>,
    total_pages: Signal<usize>,
    on_page_change: impl Fn(usize) + 'static + Copy + Send,
) -> impl IntoView {
    view! {
        <div class="pagination">
            {move || {
                let total = total_pages.get();
                let current = current_page.get();

                if total <= 1 {
                    return view! { <div></div> }.into_any();
                }

                // Build page numbers to display (max 5 pages centered on current)
                let start = if current > 2 { current - 2 } else { 1 };
                let end = std::cmp::min(start + 4, total);
                // Adjust start if end is limited
                let start = if end >= 5 { end - 4 } else { start };
                let pages: Vec<usize> = (start..=end).collect();

                let has_prev = current > 1;
                let has_next = current < total;

                view! {
                    <div class="pagination-controls">
                        <button
                            class="page-btn"
                            disabled=move || !has_prev
                            on:click=move |_| {
                                let c = current_page.get();
                                if c > 1 { on_page_change(c - 1); }
                            }
                        >
                            "前へ"
                        </button>

                        {pages.into_iter().map(|p| {
                            let is_active = p == current;
                            view! {
                                <button
                                    class=if is_active { "page-btn active" } else { "page-btn" }
                                    on:click=move |_| on_page_change(p)
                                >
                                    {p.to_string()}
                                </button>
                            }
                        }).collect_view()}

                        <button
                            class="page-btn"
                            disabled=move || !has_next
                            on:click=move |_| {
                                let c = current_page.get();
                                let t = total_pages.get();
                                if c < t { on_page_change(c + 1); }
                            }
                        >
                            "次へ"
                        </button>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
