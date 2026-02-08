# Stage 1: Build
FROM rustlang/rust:nightly AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos

WORKDIR /app
COPY . .

RUN cargo leptos build --release

# Stage 2: Runtime
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/meilisearch-search-app ./
COPY --from=builder /app/target/site ./target/site

ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000

CMD ["./meilisearch-search-app"]
