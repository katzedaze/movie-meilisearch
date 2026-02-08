# Meilisearch 検索Webサービス 開発計画

## Context

映画・書籍のカタログデータを Meilisearch で全文検索できる Web サービスを新規開発する。
Rust フルスタック構成 (Leptos + Actix-web) を採用し、日本語検索・ファセット検索に対応する。

## 技術スタック

| レイヤー | 技術 |
|---------|------|
| Frontend | Leptos 0.8 (Rust WASM, SSR対応) |
| Backend | Actix-web 4 (leptos_actix 統合) |
| 検索エンジン | Meilisearch v1.15 |
| インフラ | Docker Compose |
| ビルドツール | cargo-leptos |

## プロジェクト構成

```
meilisearch/
├── Cargo.toml
├── rust-toolchain.toml
├── docker-compose.yml
├── Dockerfile
├── .env
├── docs/
│   └── Plan.md
├── assets/
│   └── favicon.ico
├── style/
│   └── main.scss
├── seed/
│   ├── movies.json
│   └── books.json
└── src/
    ├── lib.rs
    ├── main.rs
    ├── app.rs
    ├── api.rs
    ├── model/
    │   ├── mod.rs
    │   ├── movie.rs
    │   ├── book.rs
    │   └── search.rs
    ├── server/
    │   ├── mod.rs
    │   ├── meilisearch.rs
    │   └── seed.rs
    ├── pages/
    │   ├── mod.rs
    │   ├── home.rs
    │   ├── detail.rs
    │   └── not_found.rs
    └── components/
        ├── mod.rs
        ├── search_bar.rs
        ├── search_results.rs
        ├── result_card.rs
        ├── facet_panel.rs
        └── pagination.rs
```

## データモデル

### Movie
- id, title, title_en, description, director, year, genres, rating, poster_url, language

### Book
- id, title, title_en, description, author, year, genres, rating, cover_url, language, pages

### Meilisearch インデックス設定
- **Searchable**: title, title_en, description, director/author, genres
- **Filterable**: genres, year, rating, language
- **Sortable**: year, rating, title

## API (#[server] 関数)

| 関数 | 用途 |
|------|------|
| search_items | 検索 (ファセット付き) |
| get_movie / get_book | 詳細取得 |
| create_movie / create_book | 新規作成 |
| update_movie / update_book | 更新 |
| delete_movie / delete_book | 削除 |
| seed_data | サンプルデータ投入 |
| get_facets | ファセット情報取得 |

## 検証方法

1. `docker compose up` で起動
2. http://localhost:3000 アクセス
3. 「Seed Data」ボタンでデータ投入
4. 日本語/英語で検索
5. ファセットフィルタ動作確認
6. 詳細ページ遷移確認
7. ページネーション確認
