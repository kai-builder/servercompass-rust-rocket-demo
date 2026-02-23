# syntax=docker/dockerfile:1
#
# Optimised three-stage Rust build using cargo-chef for dependency caching.
# Stage 1 (chef-prepare) and Stage 2 (cargo-chef cook) are cached as long
# as Cargo.toml / Cargo.lock don't change - only Stage 3 recompiles on
# source changes.

# ── Stage 1: compute the dependency recipe ───────────────────────────────────
FROM rust:latest AS chef

RUN cargo install cargo-chef --locked
WORKDIR /app

# ── Stage 2: resolve and pre-build dependencies ───────────────────────────────
FROM chef AS planner

# Only copy manifests needed to produce the recipe
COPY Cargo.toml Cargo.lock ./
# Create a minimal dummy main so cargo can resolve the full dep tree
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 3: compile application code ────────────────────────────────────────
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Cook (download + compile) dependencies only - cached until Cargo.toml changes
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy real source and build the binary (only app code is recompiled)
COPY . .
RUN cargo build --release

# ── Stage 4: minimal runtime image ───────────────────────────────────────────
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
