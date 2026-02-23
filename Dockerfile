# syntax=docker/dockerfile:1
#
# Two-stage Rust build for Rocket.
# Stage 1 (builder): compile on rust:1.83-slim.
# Stage 2 (runtime): minimal debian:bookworm-slim with just the binary.

# ── Stage 1: build ──────────────────────────────────────────────────────────
FROM rust:1.83-slim AS builder

WORKDIR /app

# Copy manifests and create dummy main for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release 2>/dev/null || true

# Copy real source and rebuild (only app code recompiles)
COPY . .
RUN touch src/main.rs && cargo build --release

# ── Stage 2: runtime ───────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/servercompass-rust-rocket-demo .

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -qO- http://localhost:8000/health || exit 1

CMD ["./servercompass-rust-rocket-demo"]
