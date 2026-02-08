#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use meilisearch_search_app::app::App;

    // Explicitly register server functions (inventory crate may not work in all environments)
    server_fn::actix::register_explicit::<meilisearch_search_app::api::SearchItems>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::GetMovie>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::GetBook>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::CreateMovie>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::CreateBook>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::UpdateMovie>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::UpdateBook>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::DeleteMovie>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::DeleteBook>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::SeedData>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::GetFacets>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::SearchWebAndImport>();
    server_fn::actix::register_explicit::<meilisearch_search_app::api::GetWebResult>();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    println!("Starting server at http://{}", addr);

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="ja">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone()/>
                                <HydrationScripts options=leptos_options.clone()/>
                                <leptos_meta::MetaTags/>
                            </head>
                            <body>
                                <meilisearch_search_app::app::App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .default_service(web::route().to(not_found))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
async fn not_found() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>404 - Meilisearch 検索</title>
<link rel="stylesheet" href="/pkg/meilisearch-search-app.css">
</head>
<body>
<nav class="navbar"><div class="navbar-inner"><a href="/" class="logo">Meilisearch 検索</a></div></nav>
<main class="main-content">
<div class="not-found">
<h1>404</h1>
<p>ページが見つかりませんでした</p>
<a href="/" class="back-link">ホームに戻る</a>
</div>
</main>
</body>
</html>"#,
        )
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
