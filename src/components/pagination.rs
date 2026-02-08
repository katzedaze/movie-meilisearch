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

                // Build page numbers to display
                let mut pages: Vec<usize> = Vec::new();
                let start = if current > 2 { current - 2 } else { 1 };
                let end = std::cmp::min(start + 4, total);
                for i in start..=end {
                    pages.push(i);
                }

                view! {
                    <div class="pagination-controls">
                        <button
                            class="page-btn"
                            disabled=move || current_page.get() <= 1
                            on:click=move |_| {
                                let c = current_page.get();
                                if c > 1 { on_page_change(c - 1); }
                            }
                        >
                            "前へ"
                        </button>

                        {pages.into_iter().map(|p| {
                            view! {
                                <button
                                    class=move || if current_page.get() == p { "page-btn active" } else { "page-btn" }
                                    on:click=move |_| on_page_change(p)
                                >
                                    {p}
                                </button>
                            }
                        }).collect_view()}

                        <button
                            class="page-btn"
                            disabled=move || current_page.get() >= total_pages.get()
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
