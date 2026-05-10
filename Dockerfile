# =============================================================================
# STAGE 1: Builder - Use Alpine that brings native musl
# =============================================================================
FROM rust:alpine AS builder

# Install dependencies to compile and SSL certificates
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    ca-certificates

WORKDIR /app

# Force full static compilation ONLY for the target, not for host proc-macros
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-feature=+crt-static"
ENV OPENSSL_STATIC=1

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Build the real application
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# =============================================================================
# STAGE 2: Runtime - scratch (empty image, only the binary and certificates)
# =============================================================================
FROM scratch AS runtime

# Copy SSL certificates (necessary for request/resend in HTTPS)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the compiled static binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vipsa_backend /app/vipsa_backend

EXPOSE 8080

ENTRYPOINT ["/app/vipsa_backend"]
