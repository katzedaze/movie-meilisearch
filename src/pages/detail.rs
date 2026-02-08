use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::api::{get_book, get_movie};
use crate::model::book::Book;
use crate::model::movie::Movie;

#[component]
pub fn DetailPage(index: String) -> impl IntoView {
    let params = use_params_map();
    let is_movie = index == "movies";

    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or(0)
    };

    if is_movie {
        let movie_resource = Resource::new(move || id(), |id| async move { get_movie(id).await });

        view! {
            <div class="detail-page">
                <a href="/" class="back-link">"← 検索に戻る"</a>
                <Suspense fallback=move || view! { <div class="loading">"読み込み中..."</div> }>
                    {move || {
                        movie_resource.get().map(|result| {
                            match result {
                                Ok(movie) => view! { <MovieDetail movie=movie/> }.into_any(),
                                Err(e) => view! {
                                    <div class="error">"エラー: "{e.to_string()}</div>
                                }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        }
        .into_any()
    } else {
        let book_resource = Resource::new(move || id(), |id| async move { get_book(id).await });

        view! {
            <div class="detail-page">
                <a href="/" class="back-link">"← 検索に戻る"</a>
                <Suspense fallback=move || view! { <div class="loading">"読み込み中..."</div> }>
                    {move || {
                        book_resource.get().map(|result| {
                            match result {
                                Ok(book) => view! { <BookDetail book=book/> }.into_any(),
                                Err(e) => view! {
                                    <div class="error">"エラー: "{e.to_string()}</div>
                                }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        }
        .into_any()
    }
}

#[component]
fn MovieDetail(movie: Movie) -> impl IntoView {
    let rating = format!("{:.1}", movie.rating);
    let title_en = movie.title_en.clone();

    view! {
        <article class="detail-card">
            <div class="detail-header">
                <h1>{movie.title.clone()}</h1>
                {title_en.map(|en| view! {
                    <p class="detail-title-en">{en}</p>
                })}
            </div>
            <div class="detail-meta">
                <span class="detail-type">"映画"</span>
                <span class="detail-rating">{format!("★ {rating}")}</span>
                <span class="detail-year">{movie.year.to_string()}</span>
                <span class="detail-lang">{movie.language.clone()}</span>
            </div>
            <div class="detail-info">
                <div class="info-row">
                    <span class="info-label">"監督"</span>
                    <span class="info-value">{movie.director.clone()}</span>
                </div>
            </div>
            <div class="detail-genres">
                {movie.genres.into_iter().map(|g| view! {
                    <span class="genre-tag">{g}</span>
                }).collect_view()}
            </div>
            <div class="detail-description">
                <h2>"あらすじ"</h2>
                <p>{movie.description.clone()}</p>
            </div>
        </article>
    }
}

#[component]
fn BookDetail(book: Book) -> impl IntoView {
    let rating = format!("{:.1}", book.rating);
    let title_en = book.title_en.clone();
    let pages = book.pages;

    view! {
        <article class="detail-card">
            <div class="detail-header">
                <h1>{book.title.clone()}</h1>
                {title_en.map(|en| view! {
                    <p class="detail-title-en">{en}</p>
                })}
            </div>
            <div class="detail-meta">
                <span class="detail-type">"書籍"</span>
                <span class="detail-rating">{format!("★ {rating}")}</span>
                <span class="detail-year">{book.year.to_string()}</span>
                <span class="detail-lang">{book.language.clone()}</span>
            </div>
            <div class="detail-info">
                <div class="info-row">
                    <span class="info-label">"著者"</span>
                    <span class="info-value">{book.author.clone()}</span>
                </div>
                {pages.map(|p| view! {
                    <div class="info-row">
                        <span class="info-label">"ページ数"</span>
                        <span class="info-value">{format!("{p} ページ")}</span>
                    </div>
                })}
            </div>
            <div class="detail-genres">
                {book.genres.into_iter().map(|g| view! {
                    <span class="genre-tag">{g}</span>
                }).collect_view()}
            </div>
            <div class="detail-description">
                <h2>"概要"</h2>
                <p>{book.description.clone()}</p>
            </div>
        </article>
    }
}
