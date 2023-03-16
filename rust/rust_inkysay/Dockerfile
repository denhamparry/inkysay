# Use the official Rust image as the base image
FROM cgr.dev/chainguard/rust as build
WORKDIR /usr/src/rust_inkysay
COPY . .
RUN cargo build --release

FROM cgr.dev/chainguard/cc-dynamic
COPY --from=build /usr/src/rust_inkysay/target/release/rust_inkysay /usr/local/bin/rust_inkysay
ENTRYPOINT ["rust_inkysay"]
