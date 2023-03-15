# Use the official Rust image as the base image
FROM rust:1.58.0 as build

# Set the working directory
WORKDIR /usr/src/rust_inkysay

# Copy the project files
COPY . .

# Compile the application
RUN cargo build --release

# Create a new stage using the debian slim image
FROM debian:buster-slim

# Install dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=build /usr/src/rust_inkysay/target/release/rust_inkysay /usr/local/bin/rust_inkysay

# Set the entry point
ENTRYPOINT ["rust_inkysay"]
