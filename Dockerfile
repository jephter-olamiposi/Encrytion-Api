# Stage 1: Build Stage
FROM rust:1.72-slim-buster AS builder

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="${CARGO_HOME}/bin:${PATH}"
ENV RUSTFLAGS="-C target-cpu=native"

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || { echo "Build failed during dependency caching step"; exit 1; }

COPY . .

RUN cargo build --release || { echo "Build failed during final compilation"; exit 1; }

# Stage 2: Runtime Stage
FROM debian:buster-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/secure_encryption_api .

RUN chmod +x /app/secure_encryption_api

EXPOSE 8080

CMD ["./secure_encryption_api"]
