# Reproducible production image: web server, worker, and WASM client artifacts.
FROM rust:1.98-slim AS builder
RUN rustup target add wasm32-unknown-unknown \
    && apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN cargo build --release -p momentum-web --bin momentum-server --bin momentum-worker \
    && cargo xtask web-build

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 postgresql-client \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home momentum
WORKDIR /app
COPY --from=builder /app/target/release/momentum-server /usr/local/bin/momentum-server
COPY --from=builder /app/target/release/momentum-worker /usr/local/bin/momentum-worker
COPY --from=builder /app/target/wasm32-unknown-unknown/debug /app/pkg
COPY --from=builder /app/crates/web/assets /app/assets
COPY scripts/backup.sh /app/scripts/backup.sh
USER momentum
ENV MOMENTUM_BIND=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["momentum-server"]
