FROM lukemathwalker/cargo-chef:latest-rust-1.63 AS chef

WORKDIR /app

FROM chef AS planner

COPY ./api .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application

COPY ./api .

RUN cargo build --release --bin api

FROM rust:1.63-buster AS runtime

RUN apt-get update 
RUN apt-get install -y libpq-dev pkg-config libssl-dev curl
RUN cargo install sqlx-cli --no-default-features --features postgres native-tls

WORKDIR /app

COPY ./api/migrations ./migrations

COPY --from=builder /app/target/release/api /usr/local/bin

ENTRYPOINT ["sh", "-c", "cargo sqlx migrate run && /usr/local/bin/api"]
