# =========================
# Build stage
# =========================
FROM rust:1.85-bookworm AS builder

WORKDIR /app

# (Opcional) dependencias típicas para compilar crates comunes (sqlx, openssl, etc.)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# Cache de dependencias
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Código real
COPY . .
RUN cargo build --release

# =========================
# Runtime stage (liviano)
# =========================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Si NO quieres nada extra, puedes quitar esto.
# (Pero suele ser necesario si tu app hace HTTPS a futuro)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copia el binario (ajusta "backend" al nombre real de tu binario)
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080
ENV HOST=0.0.0.0
ENV PORT=8080

CMD ["/usr/local/bin/backend"]

