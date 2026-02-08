# Meilisearch 検索

映画・書籍のカタログデータを Meilisearch で全文検索できる Web サービス。
日本語・英語の混合データに対応し、ファセットフィルタ (ジャンル・年・評価) やソート、ページネーションを備えています。
さらに SearXNG 連携により、ローカルに存在しないデータを Web 検索して取り込む機能を搭載しています。

![Top Page](docs/evidence/tc01-top-page.png)

## 主な機能

- **全文検索** — 日本語・英語のタイトル / あらすじ / 監督名・著者名をリアルタイム検索 (300ms デバウンス)
- **ファセットフィルタ** — ジャンル (件数付きチェックボックス)、年範囲、最低評価スライダー
- **ソート** — 関連度 / 年 / 評価 / タイトル順
- **インデックス切替** — 映画 (30 件)・書籍 (30 件)・Web をワンクリックで切り替え
- **Web 検索取り込み** — 検索結果が 0 件の場合、SearXNG 経由で Web 検索し結果を Meilisearch に永続化
- **詳細ページ** — 各作品のタイトル / 英題 / 監督・著者 / 年 / ジャンル / 評価 / あらすじを表示。Web 結果は元 URL リンク付き
- **ページネーション** — 12 件ごとにページ分割
- **SSR + WASM** — サーバーサイドレンダリングによる高速な初期表示 + クライアント側ハイドレーション
- **ダークテーマ** — インディゴを基調としたモダンなダーク UI
- **レスポンシブ** — デスクトップ / タブレット / モバイル対応

| 検索 + フィルタ | 書籍インデックス |
|:---:|:---:|
| ![Genre Filter](docs/evidence/tc05-genre-filter.png) | ![Index Switch](docs/evidence/tc09-index-switch.png) |

| ページネーション | 404 ページ |
|:---:|:---:|
| ![Pagination](docs/evidence/tc11-pagination.png) | ![404](docs/evidence/tc15-not-found.png) |

## 技術スタック

| レイヤー | 技術 | バージョン |
|---|---|---|
| フロントエンド | [Leptos](https://github.com/leptos-rs/leptos) (SSR + WASM hydration) | 0.8 |
| HTTP サーバー | [Actix-web](https://actix.rs/) (leptos_actix 統合) | 4 |
| 検索エンジン | [Meilisearch](https://www.meilisearch.com/) | v1.15 |
| Web 検索 | [SearXNG](https://docs.searxng.org/) (JSON API) | latest |
| 検索 SDK | meilisearch-sdk (SSR only) | 0.32 |
| HTTP クライアント | reqwest (SSR only) | 0.12 |
| ユーティリティ | leptos-use (`signal_debounced`) | 0.18 |
| ビルドツール | cargo-leptos (nightly Rust) | 0.3 |
| インフラ | Docker Compose | - |

## クイックスタート

### 前提条件

- Docker & Docker Compose

### 環境変数の設定

```bash
cp .env.sample .env
```

必要に応じて `.env` 内の値を変更してください。

### 起動

```bash
docker compose up
```

以下のサービスが起動します:

| サービス | URL | 説明 |
|---|---|---|
| Web アプリ | http://localhost:3000 | メインの検索 UI |
| Meilisearch | http://localhost:7700 | 全文検索エンジン |
| SearXNG | (内部のみ) | Web 検索メタエンジン |

### 使い方

1. ブラウザで http://localhost:3000 を開く
2. **Seed Data** ボタンをクリックしてサンプルデータを投入 (映画 30 件 + 書籍 30 件)
3. 検索バーにキーワードを入力 (例: `宮崎`, `fantasy`, `SF`)
4. ファセットパネルでジャンル / 年 / 評価を絞り込み
5. **映画** / **書籍** / **Web** ボタンでインデックスを切替
6. カードをクリックして詳細ページを表示
7. 検索結果が 0 件の場合、**Web検索して取り込む** ボタンで SearXNG 経由の Web 検索結果を取り込み
8. 取り込んだ結果は **Web** タブでいつでも再検索可能

## プロジェクト構成

```
src/
├── lib.rs              # モジュール宣言 + hydrate() エントリ
├── main.rs             # Actix-web サーバー起動 (SSR)
├── app.rs              # ルート App コンポーネント + Router
├── api.rs              # #[server] 関数 (検索・CRUD・seed・facets・Web取り込み)
├── model/              # データモデル (Movie, Book, WebResult, SearchResponse 等)
├── server/             # SSR 専用 (Meilisearch クライアント, SearXNG クライアント, seed)
├── pages/              # ページコンポーネント (home, detail, not_found)
└── components/         # UI コンポーネント (search_bar, facet_panel 等)

seed/                   # サンプル JSON データ (映画 30 件, 書籍 30 件)
searxng/                # SearXNG 設定 (settings.yml)
style/main.scss         # SCSS スタイル (ダークテーマ)
tests/                  # ブラウザ自動テスト (Puppeteer)
docs/evidence/          # テストスクリーンショット + 検証レポート
```

## 開発

### ローカル開発 (cargo-leptos)

```bash
# Meilisearch のみ起動
docker compose up meilisearch

# 開発サーバー起動 (ホットリロード)
cargo leptos watch
```

### ビルド確認

```bash
# SSR 側
cargo check --features ssr

# WASM 側
cargo check --target wasm32-unknown-unknown --features hydrate --no-default-features
```

### Docker ビルド

```bash
docker compose build app
docker compose up
```

## API

Leptos の `#[server]` マクロで実装されたサーバー関数:

| 関数 | 用途 |
|---|---|
| `search_items` | ファセット検索 + フィルタ + ソート + ページネーション (映画・書籍・Web 対応) |
| `get_movie` / `get_book` / `get_web_result` | 詳細取得 |
| `create_movie` / `create_book` | 新規作成 |
| `update_movie` / `update_book` | 更新 |
| `delete_movie` / `delete_book` | 削除 |
| `seed_data` | サンプルデータ一括投入 |
| `get_facets` | ファセット情報取得 (映画・書籍・Web 対応) |
| `search_web_and_import` | SearXNG で Web 検索し結果を Meilisearch に取り込み |

## サンプルデータ

### 映画 (30 件)

宮崎駿作品 (千と千尋の神隠し, もののけ姫, となりのトトロ 等)、クリストファー・ノーラン作品 (インターステラー, ダークナイト 等)、その他名作を日英混合で収録。

### 書籍 (30 件)

村上春樹作品 (ノルウェイの森, 海辺のカフカ, 1Q84 等)、夏目漱石、トールキン、オーウェル等の名著を日英混合で収録。

## Web 検索取り込み (SearXNG 連携)

ローカルの Meilisearch インデックスに存在しないデータを Web から取り込む機能です。

### フロー

```
ユーザー検索 → Meilisearch (0件) → "Web検索して取り込む" ボタン表示
  → SearXNG API 呼出 → 結果を Meilisearch `web` インデックスに投入
  → 即座に検索結果として表示（以降も永続的に検索可能）
```

- 取り込んだ結果は Meilisearch の `web` インデックスに永続化されます
- **Web** タブに切り替えると、過去に取り込んだ結果をいつでも検索できます
- Web 結果の詳細ページでは元の URL へのリンクが表示されます

### SearXNG 設定

SearXNG の設定ファイルは `searxng/settings.yml` にあります。Docker Compose で自動的にマウントされます。

## テスト

Puppeteer による自動ブラウザテスト (15 シナリオ):

```bash
# アプリが起動している状態で実行
node tests/browser-verify.mjs
```

テスト結果のスクリーンショットは `docs/evidence/` に、レポートは `docs/evidence/REPORT.md` に出力されます。

## 環境変数

| 変数 | デフォルト | 説明 |
|---|---|---|
| `MEILI_URL` | `http://localhost:7700` | Meilisearch の接続先 URL |
| `MEILI_MASTER_KEY` | `masterKey` | Meilisearch のマスターキー |
| `SEARXNG_URL` | `http://searxng:8080` | SearXNG の接続先 URL |

## ライセンス

MIT
