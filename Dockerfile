FROM rust:1.82 AS builder
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/suarakan-be /usr/local/bin/

WORKDIR /usr/local/bin
EXPOSE 80

CMD ["./suarakan-be"]
