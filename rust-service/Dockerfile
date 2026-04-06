# ── Build stage ────────────────────────────────────────────────────────────────
FROM rust:1.77-slim-bookworm AS builder
WORKDIR /app

# Cache dependencies separately from source
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build the real binary
COPY src ./src
RUN cargo build --release

# ── Runtime stage ───────────────────────────────────────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/logforge .
EXPOSE 3001 9090
ENV RUST_LOG=info
CMD ["./logforge"]
