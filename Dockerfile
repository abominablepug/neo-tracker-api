FROM rust:1.77 AS builder
WORKDIR /app

COPY backend/ ./backend/
WORKDIR /app/backend

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend/target/release/neo-tracker-api /usr/local/bin/

COPY frontend/ ./frontend/

EXPOSE 8080

CMD ["neo-tracker-api"]
