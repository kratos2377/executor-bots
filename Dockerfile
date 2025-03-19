FROM rust:1.85-slim as builder

WORKDIR /app
COPY . .
RUN apt-get update && \
    apt-get install -y \
    g++ \
    gcc \
    make \
    cmake \
    pkg-config \
    libssl-dev



RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/executor-bots /usr/local/bin/
CMD ["./executor-bots"]