# Meilisearch 検索 Web サービス — 作業リスト

## Phase 1: プロジェクト基盤

- [x] `Cargo.toml` 作成 (Leptos 0.8 + Actix-web 4 + meilisearch-sdk 0.32)
- [x] `rust-toolchain.toml` 作成 (nightly + wasm32-unknown-unknown)
- [x] ディレクトリ構造作成 (`src/`, `seed/`, `style/`, `assets/`, `docs/`)
- [x] `src/lib.rs` — モジュール宣言 + hydrate エントリ
- [x] `src/main.rs` — Actix-web サーバー起動 (SSR)
- [x] `src/app.rs` — ルート App コンポーネント + Router 定義
- [x] `cargo check --features ssr` 通過
- [x] `cargo check --target wasm32-unknown-unknown --features hydrate` 通過

## Phase 2: データモデル + Meilisearch クライアント

- [x] `src/model/movie.rs` — Movie 構造体定義
- [x] `src/model/book.rs` — Book 構造体定義
- [x] `src/model/search.rs` — SearchResponse / SearchHit / FacetInfo DTO
- [x] `src/server/meilisearch.rs` — クライアントシングルトン + フィルタ構築
- [x] `src/server/seed.rs` — JSON → Meilisearch 投入ロジック
- [x] `seed/movies.json` — サンプル映画データ (30 件, 日英混合)
- [x] `seed/books.json` — サンプル書籍データ (30 件, 日英混合)

## Phase 3: Docker Compose + Meilisearch 起動

- [x] `docker-compose.yml` 作成 (Meilisearch v1.15 + app)
- [x] `.env` 作成 (MEILI_MASTER_KEY)
- [x] `docker compose up meilisearch` で Meilisearch 単体起動確認
- [x] Meilisearch ヘルスチェック (`curl http://localhost:7700/health`) 確認

## Phase 4: サーバー関数 (API)

- [x] `search_items` — ファセット検索 + フィルタ + ソート + ページネーション
- [x] `get_movie` / `get_book` — 詳細取得
- [x] `create_movie` / `create_book` — 新規作成
- [x] `update_movie` / `update_book` — 更新
- [x] `delete_movie` / `delete_book` — 削除
- [x] `seed_data` — サンプルデータ一括投入
- [x] `get_facets` — ファセット情報取得

## Phase 5: 検索 UI

- [x] `SearchBar` — デバウンス付き入力 + インデックス切替 (映画/書籍)
- [x] `SearchResults` — 検索結果一覧 + ローディング / エラー状態
- [x] `ResultCard` — 結果カード (タイトル, 著者/監督, 年, ジャンル, 評価)
- [x] `HomePage` — リアクティブ検索フロー構築 (300ms デバウンス)
- [x] `style/main.scss` — ダークテーマ基本スタイリング

## Phase 6: ファセットフィルタ

- [x] `FacetPanel` — ジャンルチェックボックス (件数表示)
- [x] `FacetPanel` — 年範囲フィルタ (最小/最大)
- [x] `FacetPanel` — 評価スライダー (最低評価)
- [x] `FacetPanel` — ソートセレクト (関連度/年/評価/タイトル)
- [x] フィルタ選択 → 検索シグナルへの連携
- [x] フィルタクリアボタン

## Phase 7: 詳細ページ + ページネーション

- [x] `MovieDetail` コンポーネント (映画詳細表示)
- [x] `BookDetail` コンポーネント (書籍詳細表示)
- [x] `Pagination` コンポーネント (前へ/次へ + ページ番号)
- [x] `NotFoundPage` (404 ページ)

## Phase 8: Docker 化

- [x] `Dockerfile` 作成 (マルチステージビルド)
- [x] `.dockerignore` 作成
- [x] `docker compose build` 成功確認
- [x] `docker compose up` でエンドツーエンド動作確認

## Phase 9: 仕上げ

- [x] エラーバウンダリ / ローディング状態の UI 実装
- [x] レスポンシブ CSS (768px / 480px ブレークポイント)
- [x] SEO メタタグ (`<Title>`, `<Meta>`)
- [x] favicon.ico を実際のアイコンに差し替え

## Phase 10: chrome-devtools-mcp の設定

- [x] chrome-devtools-mcp をプロジェクトで使えるように設定する
- [x] `.claude/settings.json` に MCP サーバー設定を追加
- [x] ブラウザ起動 + MCP 接続の動作確認 (Puppeteer で代替実施)

## Phase 11: Agent Skill 「browser-verify」作成

- [x] `.claude/skills/browser-verify/` ディレクトリ作成
- [x] `SKILL.md` を作成 — Skill 定義ファイル
  - [x] chrome-devtools-mcp を使ったブラウザ自動操作手順
  - [x] 正常系テストシナリオ定義
    - [x] トップページ表示
    - [x] Seed Data ボタンでデータ投入
    - [x] 日本語検索 (例: "宮崎")
    - [x] 英語検索 (例: "fantasy")
    - [x] ジャンルフィルタ選択 → 結果絞り込み
    - [x] 年範囲フィルタ → 結果絞り込み
    - [x] 評価フィルタ → 結果絞り込み
    - [x] ソート切り替え
    - [x] インデックス切替 (映画 ⇔ 書籍)
    - [x] 詳細ページ遷移
    - [x] ページネーション動作
    - [x] 検索に戻るリンク
  - [x] 異常系テストシナリオ定義
    - [x] 空クエリ → "検索キーワードを入力してください" 表示
    - [x] 該当なしクエリ → "結果が見つかりませんでした" 表示
    - [x] 存在しない URL → 404 ページ表示
  - [x] スクリーンショット保存ルール (`docs/evidence/` 配下)
  - [x] 検証レポート生成ルール (`docs/evidence/REPORT.md`)

## Phase 12: Skill を使った自動検証

- [x] Docker Compose で全スタック起動
- [x] Puppeteer (Headless Chrome) で自動検証を実行 — **15/15 PASS**
- [x] 検証レポート (`docs/evidence/REPORT.md`) 生成済み
- [x] スクリーンショット 15 枚が `docs/evidence/` に保存済み
- [x] 不具合修正 → 再検証 (GLIBC / ServerFn / 404 / genres デシリアライズ の 4 件を修正)

## Phase 13: SearXNG Web 検索取り込み機能

### インフラ
- [x] `Cargo.toml` — `reqwest` 依存追加 (SSR feature)
- [x] `docker-compose.yml` — SearXNG サービス追加
- [x] `searxng/settings.yml` — SearXNG 設定 (JSON API 有効化)
- [x] `.env.sample` — `SEARXNG_URL` 追加

### データモデル
- [x] `src/model/web_result.rs` — WebResult 構造体 (URL ハッシュ ID)
- [x] `src/model/mod.rs` — `pub mod web_result` 追加

### サーバーロジック
- [x] `src/server/searxng.rs` — SearXNG HTTP クライアント (reqwest シングルトン)
- [x] `src/server/mod.rs` — `pub mod searxng` 追加
- [x] `src/server/meilisearch.rs` — `configure_web_index()` 追加

### API
- [x] `search_web_and_import` — SearXNG 検索 → web インデックスに投入 → SearchResponse 返却
- [x] `get_web_result` — Web 結果詳細取得
- [x] `search_items` — `index == "web"` 分岐追加
- [x] `get_facets` — `index == "web"` 分岐追加
- [x] `src/main.rs` — `SearchWebAndImport`, `GetWebResult` の explicit 登録

### UI
- [x] `SearchBar` — "Web" トグルボタン (3 タブ目) 追加
- [x] `SearchResults` — 0 件時「Web検索して取り込む」ボタン表示
- [x] `ResultCard` — `index == "web"` 対応 (URL 表示, 評価非表示)
- [x] `HomePage` — `web_importing` シグナル + `on_web_import` コールバック追加
- [x] `DetailPage` — `WebResultDetail` コンポーネント追加 (元 URL リンク付き)
- [x] `app.rs` — `/web/:id` ルート追加
- [x] `style/main.scss` — `.web-import-btn`, `.web-visit-btn` 等スタイル追加

### ビルド確認
- [x] `cargo check --features ssr` 通過 (警告なし)
- [x] `cargo check --target wasm32-unknown-unknown --features hydrate` 通過
- [x] `docker compose up --build` で 3 サービス起動確認

## Phase 14: Web 検索機能のテスト + スクリーンショット

- [x] TC-16: Web タブ切替 — Web ボタン表示 + アクティブ状態
- [x] TC-17: Web 取り込みボタン表示 — 0 件時「Web検索して取り込む」ボタン
- [x] TC-18: Web 検索取り込み実行 — SearXNG 検索 → 結果カード表示
- [x] TC-19: Web タブ検索 — 取り込み済み結果の再検索 + "Web" ラベル表示
- [x] TC-20: Web 詳細ページ — `/web/:id` ルート + 元 URL リンク
- [x] Puppeteer 自動検証 — **20/20 PASS**
- [x] スクリーンショット 20 枚が `docs/evidence/` に保存済み
- [x] README.md に Web 検索スクリーンショット追加

---

**凡例:** `[x]` = 完了, `[ ]` = 未着手
