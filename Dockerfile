FROM rust:1.82 as builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/suarakan-be /usr/local/bin/suarakan-be

COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/app/diesel.toml /usr/local/bin/

WORKDIR /usr/local/bin

EXPOSE 80

CMD ["suarakan-be"]