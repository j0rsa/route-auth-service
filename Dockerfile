#--- Build
FROM ekidd/rust-musl-builder:stable-openssl11 AS cargo-build

COPY Cargo.toml Cargo.toml
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/github_oauth_service*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

#--- Run
FROM alpine:3.10

LABEL authors="red.avtovo@gmail.com"
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/github-oauth-service /opt/
ENV RUST_LOG=info
RUN apk add --no-cache ca-certificates && update-ca-certificates

CMD ["/opt/github-oauth-service"]