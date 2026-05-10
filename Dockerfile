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

ENV OPENSSL_STATIC=1

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN TARGET=$(rustc -vV | sed -n 's|host: ||p') && \
    mkdir -p .cargo && \
    echo "[target.$TARGET]" > .cargo/config.toml && \
    echo 'rustflags = ["-C", "target-feature=+crt-static"]' >> .cargo/config.toml && \
    mkdir -p src && echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    cargo build --release --target $TARGET
RUN rm -rf src

# Build the real application
COPY src ./src
RUN TARGET=$(rustc -vV | sed -n 's|host: ||p') && \
    cargo build --release --target $TARGET && \
    cp /app/target/$TARGET/release/vipsa_backend /app/vipsa_backend_static

# =============================================================================
# STAGE 2: Runtime - scratch (empty image, only the binary and certificates)
# =============================================================================
FROM scratch AS runtime

# Copy SSL certificates (necessary for request/resend in HTTPS)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the compiled static binary
COPY --from=builder /app/vipsa_backend_static /app/vipsa_backend

EXPOSE 8080

ENTRYPOINT ["/app/vipsa_backend"]
