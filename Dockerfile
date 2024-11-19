# Use a specific Rust version to avoid unexpected changes
FROM rust:1.72 AS builder

# Set a directory inside the container for the app
WORKDIR /usr/src/app

# Copy only the files needed for dependency resolution
COPY Cargo.toml Cargo.lock ./

# Pre-fetch and compile dependencies (use Docker's caching layers efficiently)
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# Copy the rest of the application files and build the app
COPY . .
RUN cargo build --release

# Use a minimal base image for running the app
FROM debian:bullseye-slim AS runtime

# Install only necessary dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory in the runtime container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/secure_encryption_api .

# Inform Docker that the container listens on port 8080
EXPOSE 8080

# Define the default command to run the app
CMD ["./secure_encryption_api"]
