use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::{detail::DetailPage, home::HomePage, not_found::NotFoundPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/meilisearch-search-app.css"/>
        <Title text="Meilisearch 検索"/>
        <Meta name="description" content="映画・書籍のカタログを全文検索できるWebサービス"/>

        <Router>
            <nav class="navbar">
                <div class="navbar-inner">
                    <a href="/" class="logo">"Meilisearch 検索"</a>
                </div>
            </nav>
            <main class="main-content">
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/movie/:id") view=move || {
                        view! { <DetailPage index="movies".to_string()/> }
                    }/>
                    <Route path=path!("/book/:id") view=move || {
                        view! { <DetailPage index="books".to_string()/> }
                    }/>
                    <Route path=path!("/web/:id") view=move || {
                        view! { <DetailPage index="web".to_string()/> }
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
