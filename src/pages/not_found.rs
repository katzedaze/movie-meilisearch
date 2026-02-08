use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <p>"ページが見つかりませんでした"</p>
            <a href="/" class="back-link">"ホームに戻る"</a>
        </div>
    }
}
