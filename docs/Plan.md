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
| Web 検索 | SearXNG (JSON API) |
| HTTP クライアント | reqwest 0.12 (SSR only) |
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
├── searxng/
│   └── settings.yml
└── src/
    ├── lib.rs
    ├── main.rs
    ├── app.rs
    ├── api.rs
    ├── model/
    │   ├── mod.rs
    │   ├── movie.rs
    │   ├── book.rs
    │   ├── search.rs
    │   └── web_result.rs
    ├── server/
    │   ├── mod.rs
    │   ├── meilisearch.rs
    │   ├── searxng.rs
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

### WebResult
- id (URL ハッシュ), title, title_en, description, url, source_engine, year, genres, rating, image_url, language, published_date

### Meilisearch インデックス設定

**movies / books インデックス:**
- **Searchable**: title, title_en, description, director/author, genres
- **Filterable**: genres, year, rating, language
- **Sortable**: year, rating, title

**web インデックス:**
- **Searchable**: title, description, url, source_engine
- **Filterable**: genres, year, rating, language
- **Sortable**: year, rating, title

## API (#[server] 関数)

| 関数 | 用途 |
|------|------|
| search_items | 検索 (ファセット付き, 映画・書籍・Web 対応) |
| get_movie / get_book / get_web_result | 詳細取得 |
| create_movie / create_book | 新規作成 |
| update_movie / update_book | 更新 |
| delete_movie / delete_book | 削除 |
| seed_data | サンプルデータ投入 |
| get_facets | ファセット情報取得 (映画・書籍・Web 対応) |
| search_web_and_import | SearXNG で Web 検索 → Meilisearch に取り込み |

## Web 検索取り込み (SearXNG 連携)

### フロー

```
ユーザー検索 → Meilisearch (0件) → "Web検索して取り込む" ボタン表示
  → SearXNG API 呼出 → 結果を Meilisearch `web` インデックスに投入
  → 即座に検索結果として表示（以降も永続的に検索可能）
```

### アーキテクチャ

- **SearXNG**: Docker Compose で起動するメタ検索エンジン (JSON API)
- **reqwest**: SSR 側で SearXNG API を呼び出す HTTP クライアント
- **web インデックス**: 取り込んだ結果を永続化する Meilisearch インデックス
- **UI**: 映画 / 書籍 / Web の 3 タブ切替、0 件時に取り込みボタン表示

## 検証方法

1. `docker compose up` で 3 サービス起動 (Meilisearch, SearXNG, app)
2. http://localhost:3000 アクセス
3. 「Seed Data」ボタンでデータ投入
4. 日本語/英語で検索
5. ファセットフィルタ動作確認
6. 詳細ページ遷移確認
7. ページネーション確認
8. ローカルにないキーワードで検索 → 0 件 + 「Web検索して取り込む」ボタン表示
9. ボタンクリック → Web 結果がカード表示
10. Web タブ切替 → 取り込み済み結果が検索可能
11. Web カードクリック → 詳細ページ (元 URL リンク付き)
