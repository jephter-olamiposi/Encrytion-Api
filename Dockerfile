# Use the Rust official image as the base
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Expose the port the app will run on
EXPOSE 8080

# Command to run the application
CMD ["./target/release/secure_encryption_api"]
EXPOSE 8080
