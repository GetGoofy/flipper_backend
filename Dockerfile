FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin axum_api_backend

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates python3-psycopg2 libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/axum_api_backend /usr/local/bin/axum_api_backend

CMD ["/usr/local/bin/axum_api_backend"]