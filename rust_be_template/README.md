# Rust Backend Template

A copy-paste scaffold for production-style Rust HTTP APIs: multi-crate Cargo workspace, Axum, SQLx + PostgreSQL, Bearer API key auth, OpenAPI/Swagger, structured logging, and correlation IDs.

Copy this folder to start a new service. It is a **standalone Cargo workspace** — run all commands from inside the project root (or from your copy after `cp -r`).

## Prerequisites

- [Rust](https://rustup.rs/) (workspace uses **edition 2024**)
- [Docker](https://www.docker.com/) and Docker Compose (local Postgres)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) for Makefile migrations:

  ```bash
  cargo install sqlx-cli --no-default-features --features rustls,postgres
  ```

- Optional: `psql` (used by `make dev` to verify `DATABASE_URL` before starting)

## Quick start

```bash
cp -r rust_be_template ~/my-new-api
cd ~/my-new-api
cp .env.example .env
make dev
```

Endpoints (default `BIND_ADDR=0.0.0.0:3000`):

| URL                                         | Description  |
| ------------------------------------------- | ------------ |
| http://localhost:3000                       | API info     |
| http://localhost:3000/health                | Health check |
| http://localhost:3000/swagger-ui            | Swagger UI   |
| http://localhost:3000/api-docs/openapi.json | OpenAPI spec |

### Create a business and call a protected route

`POST /api/v1/business` returns a one-time API key (`name` + `api_key`). Store the key immediately — only a bcrypt hash is persisted.

```bash
curl -s -X POST http://localhost:3000/api/v1/business \
  -H 'Content-Type: application/json' \
  -d '{"name":"Acme Corp"}'

# Use the api_key from the response (format: sk_live_<hex>.<secret>)
export API_KEY='sk_live_abcd1234.xxxxxxxxxxxx'

curl -s http://localhost:3000/api/v1/me \
  -H "Authorization: Bearer $API_KEY"
```

API keys use the prefix from `API_PREFIX` (default `sk_live`), e.g. `sk_live_a1b2c3d4.<url-safe-secret>`.

## Stack

| Layer         | Choice                                                                     |
| ------------- | -------------------------------------------------------------------------- |
| HTTP          | [Axum](https://github.com/tokio-rs/axum) 0.8                               |
| Async runtime | Tokio                                                                      |
| Database      | PostgreSQL 16 via [SQLx](https://github.com/launchbadge/sqlx)              |
| API docs      | [utoipa](https://github.com/juhaku/utoipa) + Swagger UI                    |
| Auth          | Bearer API keys (bcrypt-hashed at rest)                                    |
| Observability | `tracing` + JSON-friendly subscriber, HTTP trace layer, `x-correlation-id` |

## Project layout

```
rust_be_template/
├── Cargo.toml              # Workspace root
├── Makefile
├── docker-compose.yaml     # Postgres on host port 5433
├── .env.example
├── migrations/             # SQLx migrations
└── crates/
    ├── api/                # HTTP server, routes, handlers, middleware
    │   └── src/
    │       ├── main.rs
    │       ├── config.rs
    │       ├── state.rs    # AppState + query wiring
    │       ├── handlers/   # Request handlers + OpenAPI paths
    │       ├── middleware/ # e.g. AuthenticatedBusiness extractor
    │       ├── routes/     # Router, CORS, Swagger, v1 nesting
    │       ├── services/   # Domain services (stub)
    │       └── workers/    # Background jobs (stub)
    ├── db/                 # Pool, migrations runner, SQL query modules
    │   └── src/
    │       ├── config.rs
    │       ├── business_queries.rs
    │       ├── api_key_queries.rs
    │       └── api_keys.rs # Generate / hash / verify keys
    └── shared/             # Cross-crate types, errors, middleware, logging
        └── src/
            ├── errors.rs
            ├── types.rs
            ├── middleware.rs
            └── observability.rs
```

### Crates

- **api** — Binds the server, builds the Axum router, owns HTTP-specific auth (`AuthenticatedBusiness`).
- **db** — Connection pool, embedded migrations at startup, parameterized queries.
- **shared** — `AppError` / `ErrorResponse`, DTOs, correlation middleware, observability bootstrap.

## API surface (v1)

| Method | Path               | Auth           | Purpose                            |
| ------ | ------------------ | -------------- | ---------------------------------- |
| `GET`  | `/`                | —              | Version string                     |
| `GET`  | `/health`          | —              | Liveness                           |
| `POST` | `/api/v1/business` | —              | Create business + issue API key    |
| `GET`  | `/api/v1/me`       | Bearer API key | Return authenticated `business_id` |

Protected handlers use the `AuthenticatedBusiness` extractor (`middleware/auth.rs`), which validates `Authorization: Bearer <api_key>` against the database.

## Configuration

Copy `.env.example` to `.env`. Important variables:

| Variable                | Default                                             | Description                                  |
| ----------------------- | --------------------------------------------------- | -------------------------------------------- |
| `DATABASE_URL`          | `postgresql://postgres:password@localhost:5433/app` | Postgres connection string                   |
| `BIND_ADDR`             | `0.0.0.0:3000`                                      | Listen address                               |
| `API_PREFIX`            | `sk_live`                                           | Prefix for generated API keys                |
| `CORS_ALLOWED_ORIGINS`  | `*` (all origins)                                   | Comma-separated list, or `*`                 |
| `RUST_LOG`              | see `.env.example`                                  | Log filter (e.g. `info,api=debug,sqlx=warn`) |
| `DB_MAX_CONNECTIONS`    | `10`                                                | Pool size cap                                |
| `DB_MIN_CONNECTIONS`    | `0`                                                 | Pool floor                                   |
| `DB_CONNECTION_TIMEOUT` | `30`                                                | Acquire timeout (seconds)                    |

Postgres runs in Docker on **host port 5433** (container 5432) to avoid clashing with a local Postgres on 5432.

## Database

Initial migrations:

- `business` — tenant record (`id`, `name`, `created_at`)
- `api_key` — per-business keys (`key_prefix`, `key_hash`, optional `revoked_at`)

Migrations run when:

1. You invoke `make migrate` / `make dev` (via **sqlx-cli**), and
2. The API starts (`AppState` runs SQLx migrate from `migrations/`).

## Adding a new resource

Typical flow:

1. Add a migration: `make migrate-new NAME='add_widgets'`, then edit `migrations/<timestamp>_add_widgets.sql`
2. Add query methods in `crates/db/src/` (new module + `pub use` in `lib.rs`)
3. Expose queries on `AppState` in `crates/api/src/state.rs`
4. Add a handler under `crates/api/src/handlers/`
5. Register routes in `crates/api/src/routes/v1/` (and `handlers/mod.rs` if needed)
6. Register the handler in `crates/api/src/routes/mod.rs` (`#[openapi(paths(...))]` + `components(schemas(...))`)

For authenticated routes, add `AuthenticatedBusiness` as a handler parameter (see `handlers/me.rs`).

Extend `crates/api/src/services/` for orchestration and `crates/api/src/workers/` for async/background work.

## Makefile commands

| Command                           | Description                                                      |
| --------------------------------- | ---------------------------------------------------------------- |
| `make services-start`             | Start Postgres (Docker, port **5433**)                           |
| `make services-stop`              | Stop Postgres container                                          |
| `make migrate`                    | Apply migrations (`sqlx migrate run`)                            |
| `make migrate-revert`             | Revert last migration                                            |
| `make migrate-info`               | Show migration status                                            |
| `make migrate-new NAME='add_foo'` | Create a new migration file                                      |
| `make dev`                        | Start Postgres, migrate, verify DB, run API (`cargo run -p api`) |
| `make start`                      | Same as dev but without the extra `psql` connectivity check      |
| `make stop`                       | Stop Postgres                                                    |
| `make status`                     | Docker Compose status for Postgres                               |

## Testing

```bash
cargo test -p db
```

Covers API key generation format and bcrypt hash/verify round-trips (`crates/db/src/api_keys.rs`).

## What's included

- Three-crate workspace (`api`, `db`, `shared`)
- Example multi-tenant auth: business onboarding + Bearer API keys
- OpenAPI 3 + Swagger UI with bearer security scheme
- CORS, request tracing, correlation ID propagation
- Docker Compose for local Postgres
- Makefile for common dev workflows
- Stub `services/` and `workers/` modules for growth

## What's not included (add as needed)

Domain-specific resources, webhooks, message queues, background workers, idempotency keys, integration/e2e tests, and deployment manifests beyond local Docker Compose.
