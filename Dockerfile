# =============================================================================
# STAGE 1: Builder - Compilar estáticamente con musl target
# =============================================================================
FROM rust:1.88 AS builder

# Instalar dependencias para compilar con musl target y OpenSSL
RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-dev \
    libssl-dev \
    pkg-config \
    wget \
    perl \
    && rm -rf /var/lib/apt/lists/*

# Agregar target musl
RUN rustup target add x86_64-unknown-linux-musl

# Crear symlinks para las herramientas de cross-compilación que no tienen versión musl dedicada
RUN ln -s /usr/bin/ar /usr/bin/x86_64-linux-musl-ar && \
    ln -s /usr/bin/ranlib /usr/bin/x86_64-linux-musl-ranlib

# Compilar OpenSSL estáticamente para musl target
ENV OPENSSL_VERSION=3.4.1
ENV OPENSSL_DIR=/opt/openssl-musl

# Descargar y compilar OpenSSL para musl
# OPENSSL_NO_SECURE_MEMORY deshabilita el código que requiere linux/mman.h
# no-engine deshabilita engines (incluyendo e_afalg que requiere linux/version.h)
RUN wget -q https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz -O /tmp/openssl.tar.gz && \
    cd /tmp && tar -xzf openssl.tar.gz && cd openssl-${OPENSSL_VERSION} && \
    # Configure para linux-generic64 (funciona con musl libc)
    ./Configure --prefix=${OPENSSL_DIR} --cross-compile-prefix=x86_64-linux-musl- \
        no-shared no-zlib no-async no-threads no-engine \
        linux-generic64 && \
    # Build con OPENSSL_NO_SECURE_MEMORY para evitar linux/mman.h
    make -j$(nproc) \
        CFLAGS="-DL_ENDIAN -DOPENSSL_PIC -DNDEBUG -DOPENSSL_SMALL_FOOTPRINT -DOPENSSL_NO_SECURE_MEMORY" \
        build_sw && \
    make install_sw && \
    cd / && rm -rf /tmp/openssl*

# Configurar variables para cross-compilación
ENV OPENSSL_DIR=/opt/openssl-musl
ENV OPENSSL_LIB_DIR=/opt/openssl-musl/lib
ENV OPENSSL_INCLUDE_DIR=/opt/openssl-musl/include
ENV PKG_CONFIG_PATH=/opt/openssl-musl/lib/pkgconfig
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_DIR=/opt/openssl-musl
ENV OPENSSL_STATIC=1

WORKDIR /app

# Cache de dependencias primero
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Build de la aplicación real
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# =============================================================================
# STAGE 2: Runtime - scratch (imagen vacía, solo el binario)
# =============================================================================
FROM scratch AS runtime

# Copiar binario estático (no necesita ninguna librería adicional)
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vipsa_backend /app/vipsa_backend

EXPOSE 8080

ENTRYPOINT ["/app/vipsa_backend"]
