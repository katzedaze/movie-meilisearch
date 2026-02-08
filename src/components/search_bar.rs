use leptos::prelude::*;

#[component]
pub fn SearchBar(
    query: ReadSignal<String>,
    set_query: WriteSignal<String>,
    index: ReadSignal<String>,
    set_index: WriteSignal<String>,
    on_seed: impl Fn() + 'static + Clone,
    seeding: ReadSignal<bool>,
) -> impl IntoView {
    let on_seed_clone = on_seed.clone();

    view! {
        <div class="search-bar">
            <div class="search-input-wrapper">
                <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="11" cy="11" r="8"></circle>
                    <path d="M21 21l-4.35-4.35"></path>
                </svg>
                <input
                    type="text"
                    class="search-input"
                    placeholder="検索... (例: 宮崎, fantasy, SF)"
                    prop:value=query
                    on:input=move |ev| {
                        use wasm_bindgen::JsCast;
                        let target = ev.target().unwrap();
                        let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
                        set_query.set(input.value());
                    }
                />
            </div>
            <div class="search-controls">
                <div class="index-toggle">
                    <button
                        class=move || if index.get() == "movies" { "toggle-btn active" } else { "toggle-btn" }
                        on:click=move |_| set_index.set("movies".to_string())
                    >
                        "映画"
                    </button>
                    <button
                        class=move || if index.get() == "books" { "toggle-btn active" } else { "toggle-btn" }
                        on:click=move |_| set_index.set("books".to_string())
                    >
                        "書籍"
                    </button>
                    <button
                        class=move || if index.get() == "web" { "toggle-btn active" } else { "toggle-btn" }
                        on:click=move |_| set_index.set("web".to_string())
                    >
                        "Web"
                    </button>
                </div>
                <button
                    class="seed-btn"
                    on:click=move |_| on_seed_clone()
                    disabled=seeding
                >
                    {move || if seeding.get() { "投入中..." } else { "Seed Data" }}
                </button>
            </div>
        </div>
    }
}
