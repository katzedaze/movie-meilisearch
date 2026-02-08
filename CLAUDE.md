# CLAUDE.md — Meilisearch 検索 Web サービス

## プロジェクト概要

映画・書籍のカタログデータを Meilisearch で全文検索できる Web サービス。
日本語・英語の混合データに対応し、ファセットフィルタ (ジャンル・年・評価) を備える。

- URL: `http://localhost:3000`
- Meilisearch: `http://localhost:7700`

## 技術スタック

| レイヤー | 技術 | バージョン |
| --- | --- | --- |
| フレームワーク | Leptos (SSR + WASM hydration) | 0.8 |
| HTTP サーバー | Actix-web (leptos_actix 統合) | 4 |
| 検索エンジン | Meilisearch | v1.15 |
| 検索 SDK | meilisearch-sdk (SSR only) | 0.32 |
| ユーティリティ | leptos-use (`signal_debounced`) | 0.18 |
| ビルド | cargo-leptos (nightly Rust) | 0.3 |
| インフラ | Docker Compose | - |

## ビルド・実行コマンド

```bash
# SSR 側のチェック
cargo check --features ssr

# WASM 側のチェック
cargo check --target wasm32-unknown-unknown --features hydrate --no-default-features

# 開発サーバー起動 (cargo-leptos が必要)
cargo leptos watch

# Docker で全スタック起動
docker compose up

# Meilisearch のみ起動
docker compose up meilisearch
```

## ディレクトリ構造

```
src/
├── lib.rs              # モジュール宣言 + hydrate() エントリ
├── main.rs             # Actix-web サーバー起動 (SSR feature のみ)
├── app.rs              # ルート App コンポーネント + Router 定義
├── api.rs              # #[server] 関数 (検索・CRUD・seed・facets)
├── model/              # データ構造体 (SSR/WASM 両方で使用)
│   ├── movie.rs
│   ├── book.rs
│   └── search.rs       # SearchResponse, SearchHit, FacetInfo 等
├── server/             # SSR 専用ロジック (#[cfg(feature = "ssr")])
│   ├── meilisearch.rs  # クライアントシングルトン + フィルタ構築
│   └── seed.rs         # JSON → Meilisearch 投入
├── pages/              # ページコンポーネント (Router の view に対応)
│   ├── home.rs
│   ├── detail.rs
│   └── not_found.rs
└── components/         # 再利用可能な UI コンポーネント
    ├── search_bar.rs
    ├── search_results.rs
    ├── result_card.rs
    ├── facet_panel.rs
    └── pagination.rs

seed/                   # サンプルデータ JSON (include_str! で埋め込み)
style/main.scss         # SCSS スタイル (cargo-leptos が自動コンパイル)
docs/                   # ドキュメント・作業リスト・検証エビデンス
```

## Feature Gate ルール

このプロジェクトは 2 つのコンパイルターゲットを持つ。
依存と `use` 文は以下のルールに従うこと。

| 分類 | feature gate | 例 |
| --- | --- | --- |
| モデル構造体 (`model/`) | **なし** (両方で必要) | `Movie`, `Book`, `SearchResponse` |
| `#[server]` 関数の宣言 | **なし** (Leptos が自動処理) | `pub async fn search_items(...)` |
| `#[server]` 関数内の `use` | 関数本体内に書く | `use crate::server::meilisearch::get_client;` |
| SSR 専用の定数・ヘルパー | `#[cfg(feature = "ssr")]` | `const HITS_PER_PAGE`, `parse_facets()` |
| `server/` モジュール全体 | `#[cfg(feature = "ssr")]` | `lib.rs` で gate |
| SSR 専用クレート | `optional = true` + feature | `meilisearch-sdk`, `serde_json` |

**重要:** `#[server]` 関数の戻り値型 (`Movie`, `Book` 等) は WASM 側でもデシリアライズされるため、**cfg-gate してはいけない。**

## コーディング規約

### 命名規則

| 対象 | 規則 | 例 |
| --- | --- | --- |
| ファイル名 | snake_case | `search_bar.rs`, `not_found.rs` |
| 構造体 | PascalCase | `SearchResponse`, `FacetPanel` |
| コンポーネント関数 | PascalCase (`#[component]`) | `pub fn SearchBar(...)` |
| `#[server]` 関数 | snake_case | `pub async fn search_items(...)` |
| シグナル変数 | `(foo, set_foo) = signal(...)` | `(query, set_query)` |
| CSS クラス | kebab-case | `.search-bar`, `.result-card` |

### Leptos コンポーネントの書き方

```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent(
    // ReadSignal / WriteSignal で親の状態を受け取る
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    // コールバックは impl Fn + 'static + Copy + Send
    on_action: impl Fn() + 'static + Copy + Send,
    // Signal<T> で derived/computed な値を受け取る
    computed: Signal<usize>,
) -> impl IntoView {
    view! {
        // ...
    }
}
```

**遵守事項:**

- view 内では **owned String** を渡す (`value.clone()` / `format!(...)`)。`&String` は `IntoRender` を実装しないためコンパイルエラーになる
- イベントハンドラ内の DOM アクセスは `wasm_bindgen::JsCast` + `web_sys::HtmlInputElement` を使う
- `Signal::derive(move || ...)` で computed 値を作り、子コンポーネントに `Signal<T>` として渡す
- コールバック prop には `+ Send` を付ける (SSR で必要)

### `#[server]` 関数の書き方

```rust
#[server]
pub async fn my_function(arg: String) -> Result<MyResponse, ServerFnError> {
    // SSR 専用の use はここに書く
    use crate::server::meilisearch::get_client;

    let client = get_client();
    // ...
    Ok(response)
}
```

- 引数・戻り値は `Serialize + Deserialize` を実装すること
- SSR 専用の `use` 文は関数本体の先頭に書く (トップレベルに書くと WASM でエラー)
- エラーは `ServerFnError::new(format!("..."))` で返す

### Meilisearch クライアント

`server/meilisearch.rs` の `get_client()` でシングルトンを取得する。

```rust
let client = get_client();
let index = client.index("movies");
```

- 環境変数: `MEILI_URL` (デフォルト `http://localhost:7700`), `MEILI_MASTER_KEY` (デフォルト `masterKey`)
- フィルタ構築: `build_filter(&genres, year_min, year_max, rating_min)` を使う
- sort は `Vec<String>` を作ってから `Vec<&str>` に変換して渡す (ライフタイム対策)

### スタイリング

- `style/main.scss` に全スタイルを記述 (コンポーネント単位の分割はしない)
- SCSS 変数はファイル先頭に定義 (`$primary`, `$bg`, `$text` 等)
- CSS クラス名は kebab-case、コンポーネント名をプレフィックスにする (`card-title`, `facet-panel`)
- レスポンシブ: `768px` (タブレット) と `480px` (モバイル) のブレークポイント

## ファイル配置ルール

### 新しいデータモデルを追加する場合

1. `src/model/` に構造体を定義 (`#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]`)
2. `src/model/mod.rs` に `pub mod` を追加
3. Option フィールドには `#[serde(default)]` を付ける

### 新しいページを追加する場合

1. `src/pages/` にファイルを作成
2. `src/pages/mod.rs` に `pub mod` を追加
3. `src/app.rs` の `<Routes>` に `<Route path=path!("/new") view=NewPage/>` を追加

### 新しいコンポーネントを追加する場合

1. `src/components/` にファイルを作成
2. `src/components/mod.rs` に `pub mod` を追加
3. 状態は親から `ReadSignal`/`WriteSignal` で受け取る (コンポーネント内でグローバル状態を持たない)

### 新しい `#[server]` 関数を追加する場合

1. `src/api.rs` に関数を追加
2. SSR 専用ロジックが複雑なら `src/server/` にヘルパーを切り出す
3. 戻り値の型が新しいなら `src/model/` に追加 (cfg-gate しない)
4. `src/main.rs` に `server_fn::actix::register_explicit::<crate::api::FnName>()` を追加 (PascalCase)

### 新しいインデックスを追加する場合

1. `src/model/` に構造体を追加
2. `seed/` にサンプル JSON を追加
3. `src/server/seed.rs` に投入関数を追加
4. `src/server/meilisearch.rs` の `configure_index()` を拡張
5. `src/api.rs` に CRUD + 検索関数を追加

## 共通コンポーネントの使い方

### SearchBar

検索入力 + インデックス切替 + Seed ボタン。

```rust
<SearchBar
    query=query                   // ReadSignal<String>
    set_query=set_query           // WriteSignal<String>
    index=index                   // ReadSignal<String> ("movies" | "books")
    set_index=set_index           // WriteSignal<String>
    on_seed=on_seed               // impl Fn() + 'static + Clone
    seeding=seeding               // ReadSignal<bool>
/>
```

### FacetPanel

フィルタパネル。ジャンル・年範囲・評価スライダー・ソート。

```rust
<FacetPanel
    facets=facets.into()                    // Signal<Option<Result<FacetInfo, ServerFnError>>>
    selected_genres=selected_genres          // ReadSignal<Vec<String>>
    set_selected_genres=set_selected_genres  // WriteSignal<Vec<String>>
    year_min=year_min                        // ReadSignal<Option<i32>>
    set_year_min=set_year_min                // WriteSignal<Option<i32>>
    year_max=year_max                        // ReadSignal<Option<i32>>
    set_year_max=set_year_max                // WriteSignal<Option<i32>>
    rating_min=rating_min                    // ReadSignal<Option<f64>>
    set_rating_min=set_rating_min            // WriteSignal<Option<f64>>
    set_sort=set_sort                        // WriteSignal<Option<String>>
/>
```

### SearchResults

検索結果一覧。ローディング・エラー・空状態を内部でハンドリング。

```rust
<SearchResults
    results=results.into()  // Signal<Option<Result<SearchResponse, ServerFnError>>>
    loading=loading.into()  // Signal<bool>
/>
```

### ResultCard

個別の検索結果カード。`SearchHit` を受け取って描画。

```rust
<ResultCard hit=hit/>  // SearchHit (owned)
```

### Pagination

ページ切り替え UI。

```rust
<Pagination
    current_page=current_page    // Signal<usize>
    total_pages=total_pages      // Signal<usize>
    on_page_change=on_page_change  // impl Fn(usize) + 'static + Copy + Send
/>
```

## よくあるハマりどころ

| 問題 | 原因 | 対処 |
| --- | --- | --- |
| `&String: IntoRender is not satisfied` | view 内で参照を渡している | `.clone()` や `format!()` で owned String にする |
| `cannot be sent between threads safely` | コールバック prop に `Send` がない | `+ Send` を追加する |
| `type annotations needed` for `signal_debounced` | `Signal::from()` の型推論が曖昧 | `ReadSignal` を直接渡す: `signal_debounced(query, 300.0)` |
| `sort_criteria does not live long enough` | `Vec<&str>` のライフタイムが search よりも短い | `Vec<String>` を先に作り、`if` の外で `Vec<&str>` に変換する |
| SSR-only 型を cfg-gate して WASM エラー | `#[server]` の戻り値型が WASM で見えない | model 型は cfg-gate しない |
| `leptos_routes` の引数エラー | Leptos 0.8 で API が変更 | `leptos_routes(routes, app_fn)` — options は渡さない |
| `facet_distribution` の型不一致 | BTreeMap を期待しているが HashMap が返る | `HashMap<String, HashMap<String, usize>>` を使う |
| Server functions 404 / 405 | `Files::new("/", ...)` が全パスをキャッチ / inventory 未登録 | 静的ファイルは `/pkg` と `/assets` を個別にマウント。`register_explicit` で明示登録 |
| Dockerfile GLIBC エラー | ビルダーと実行イメージの GLIBC バージョン不一致 | runtime に `debian:trixie-slim` を使う |
