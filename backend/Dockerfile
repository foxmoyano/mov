# ---------- BUILDER ----------
FROM rust:1.85-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# Copia manifiestos
COPY Cargo.toml Cargo.lock ./

# Copia SOLO el código (y otros dirs necesarios)
COPY src ./src

# Verificación: si esto falla, el build no está viendo tu cambio
RUN grep -n "main(): start" src/main.rs

# Build explícito del bin
RUN cargo build --release --bin backend

# ---------- RUNTIME ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/backend"]

