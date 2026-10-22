# Reproducible production image: web server, worker, and WASM client artifacts.
FROM rust:1.98-slim AS builder
ENV RUSTUP_TOOLCHAIN=1.98.0
RUN rustup target add wasm32-unknown-unknown \
    && apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install wasm-bindgen-cli --version 0.2.128 --locked
WORKDIR /app
COPY . .
RUN cargo build --locked --release -p momentum-web --bin momentum-server --bin momentum-worker \
    && cargo build --locked --release -p momentum-web --lib --no-default-features \
        --features hydrate --target wasm32-unknown-unknown \
    && mkdir -p target/site/pkg \
    && wasm-bindgen --target web --out-dir target/site/pkg --out-name momentum-web \
        target/wasm32-unknown-unknown/release/momentum_web.wasm

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 postgresql-client \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home momentum
WORKDIR /app
COPY --from=builder /app/target/release/momentum-server /usr/local/bin/momentum-server
COPY --from=builder /app/target/release/momentum-worker /usr/local/bin/momentum-worker
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/crates/web/assets /app/assets
COPY scripts/backup.sh /app/scripts/backup.sh
USER momentum
ENV MOMENTUM_BIND_ADDR=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["momentum-server"]
