# Stage 1: Build Stage
FROM rust:1.72-slim-buster AS build

# Set environment variables for Rust compilation
ENV RUSTFLAGS="-C target-cpu=native"
ENV CARGO_HOME="/usr/local/cargo"
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# Install required dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Cache dependencies by copying only the Cargo.toml and Cargo.lock first
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to prevent rebuilding unnecessary layers
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies first
RUN cargo build 

# Copy the rest of the application code and rebuild the actual binary
COPY . .
RUN cargo build 

# Stage 2: Runtime Stage
FROM debian:buster-slim AS runtime

# Install minimal dependencies for runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/secure_encryption_api .

# Ensure the binary is executable
RUN chmod +x /app/secure_encryption_api

# Expose the application port
EXPOSE 8080

# Define the default command to run the application
CMD ["./secure_encryption_api"]
