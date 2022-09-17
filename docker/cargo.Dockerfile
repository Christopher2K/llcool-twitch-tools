FROM rust:1.63-slim-buster as base

ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait /wait
RUN chmod +x /wait

RUN apt-get update 
RUN apt-get install -y libpq-dev pkg-config libssl-dev
RUN cargo install diesel_cli --no-default-features --features postgres

FROM base as development

RUN cargo install cargo-watch

WORKDIR /api
