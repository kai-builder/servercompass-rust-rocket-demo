# Server Compass Rust Rocket Demo

> A production-ready Rust Rocket application template for self-hosting with [Server Compass](https://servercompass.app/)

A minimal Rocket server that surfaces public environment variables with a `Not set` fallback and keeps private values on the backend.

## Features

- Home page shows public env vars (`APP_NAME`, `API_URL`, `ENVIRONMENT`, `VERSION`) with `Not set` fallback.
- JSON endpoint `/api/env` returns the same public values.
- Health endpoint `/health` returns `{"status":"ok","service":"servercompass-rust-rocket-demo"}`.
- Private envs (`DATABASE_URL`, `API_SECRET_KEY`) stay server-side and are never exposed to clients.
- Defaults are loaded at startup so the app works out of the box.

## Quick Start

```bash
cd servercompass-rust-rocket-demo
cp .env.example .env  # optional; dotenv is loaded automatically

cargo run
# open http://localhost:8000 (or PORT if set)
```

## Environment Variables

Public (shown in UI and `/api/env`):

| Variable      | Default                              |
|---------------|--------------------------------------|
| `APP_NAME`    | `ServerCompass Rust Rocket`          |
| `API_URL`     | `https://api.servercompass.app`      |
| `ENVIRONMENT` | `production`                         |
| `VERSION`     | `1.0.0`                              |

Private (server-only, never sent to clients):

| Variable         | Default                                                    |
|------------------|------------------------------------------------------------|
| `DATABASE_URL`   | `postgresql://user:password@localhost:5432/servercompass`  |
| `API_SECRET_KEY` | `your-secret-key-here`                                     |

Unset public values render as `Not set` in both HTML and JSON responses.

## Endpoints

- `GET /` - HTML page listing public env vars with fallback styling
- `GET /api/env` - JSON `{ envs: [{ key, value }] }`
- `GET /health` - JSON `{ status: "ok", service: "servercompass-rust-rocket-demo" }`

## Docker

```bash
# Build
docker build -t servercompass-rust-rocket-demo .

# Run
docker run -p 8000:8000 --env-file .env.example servercompass-rust-rocket-demo

# open http://localhost:8000
```

The Dockerfile uses [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) for dependency-layer caching - only application code is recompiled on source changes, keeping incremental builds fast.

## Deploy to Your VPS

Deploy this Rust Rocket application to any VPS in minutes with [Server Compass](https://servercompass.app/) - the modern way to self host Rust applications.

Server Compass handles:
- Docker image builds on your VPS
- Environment variable management
- Container lifecycle and restarts
- Deployment from any Git branch

GitHub: [https://github.com/kai-builder/servercompass-rust-rocket-demo](https://github.com/kai-builder/servercompass-rust-rocket-demo)

---

Keywords: self host rust, deploy rust to vps, install rust rocket, rust rocket docker deployment, self-hosted rust rocket
