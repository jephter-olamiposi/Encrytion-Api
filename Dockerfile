# Stage 1: Builder Stage
FROM rust:1.72-slim-buster AS builder

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || { echo "Dependency caching failed"; exit 1; }

# Copy source code and build
COPY . .
RUN cargo build --release || { echo "Build failed"; exit 1; }

# Stage 2: Runtime Stage
FROM debian:buster-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/secure_encryption_api .

# Ensure the binary is executable
RUN chmod +x /app/secure_encryption_api

# Expose the application's port
ENV PORT=8080
EXPOSE 8080

# Set environment variables for production
ENV RUST_LOG=info

# Run the application
CMD ["./secure_encryption_api"]
