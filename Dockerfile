# Stage 1: Build Stage
FROM rust:1.72-slim-buster AS builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy only the Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || { echo "Build failed during dependency caching step"; exit 1; }

# Copy the rest of the source code
COPY . .

# Compile the Rust project in release mode
RUN cargo build --release || { echo "Build failed during final compilation"; exit 1; }

# Stage 2: Runtime Stage
FROM debian:buster-slim AS runtime

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/secure_encryption_api .

# Ensure the binary is executable
RUN chmod +x /app/secure_encryption_api

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./secure_encryption_api"]
