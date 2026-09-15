# Single image: the SvelteKit app is built first, then embedded into the
# Rust binary, which serves both the API and the web app.

FROM node:24-alpine AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend ./
RUN npm run build

FROM rust:1.98-slim-trixie AS builder
# axum-vite depends on reqwest with native TLS, which needs OpenSSL headers to
# compile (the release binary itself does not link OpenSSL).
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs ./
COPY migration ./migration
COPY src ./src
COPY --from=frontend /app/frontend/build ./frontend/build
RUN cargo build --release --locked

FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myapp-server /usr/local/bin/myapp-server
ENV BIND_ADDR=0.0.0.0:3000
EXPOSE 3000
CMD ["myapp-server"]
