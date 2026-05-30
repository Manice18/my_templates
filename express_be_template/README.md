# Express Backend Template

A minimal Node.js + Express + TypeScript starter with PostgreSQL. Copy this folder when scaffolding a new API.

## Layout

```
express_be_template/
├── db/
│   └── migrations/        # SQL migrations (applied via npm run db:migrate)
├── src/
│   ├── server.ts          # Entry point, DB connect, graceful shutdown
│   ├── app.ts             # Express app, middleware, error handler
│   ├── config/            # env (zod) + winston logger
│   ├── db/                # Pool, migration runner, query modules
│   ├── errors/            # ApiError
│   ├── middleware/        # Shared middleware
│   ├── routes/            # Route modules + index registrar
│   └── services/          # Business logic
├── docker-compose.yaml    # Local PostgreSQL
├── Makefile               # services-start, migrate, dev
├── .env.example
├── package.json
└── tsconfig.json
```

## Prerequisites

- [Node.js](https://nodejs.org/)
- [Docker](https://www.docker.com/) and Docker Compose (local Postgres)

## Quick start

```bash
cp -R express_be_template my_project_be
cd my_project_be
npm install
cp .env.example .env
make dev
```

`make dev` starts Postgres, runs migrations, and launches the API with hot reload.

Alternatively, run each step manually:

```bash
docker compose up -d postgres
npm run db:migrate
npm run dev
```

Server defaults to **http://localhost:8080**. Postgres is exposed on **localhost:5433**.

## Example endpoints

| Method | Path                | Description        |
| ------ | ------------------- | ------------------ |
| `GET`  | `/api/health`       | Health check (includes DB status) |
| `GET`  | `/api/examples`     | List example items |
| `GET`  | `/api/examples/:id` | Get one item       |

## Adding a new resource

1. Add a migration in `db/migrations/`.
2. Create query functions in `src/db/queries/`.
3. Create a service in `src/services/`.
4. Add a route module in `src/routes/` using the `makeXRouter()` factory pattern.
5. Register it in `src/routes/index.ts`.

## Scripts

| Script              | Purpose                            |
| ------------------- | ---------------------------------- |
| `npm run dev`       | tsx watch — hot reload             |
| `npm run build`     | esbuild bundle to `dist/server.js` |
| `npm start`         | Run production build               |
| `npm run check`     | TypeScript typecheck               |
| `npm run db:migrate`| Apply pending SQL migrations       |

## Make targets

| Target            | Purpose                                      |
| ----------------- | -------------------------------------------- |
| `make services-start` | Start Postgres via Docker Compose        |
| `make services-stop`  | Stop Postgres                            |
| `make migrate`        | Run `npm run db:migrate`                 |
| `make dev`            | Start Postgres, migrate, then `npm run dev` |

## Included patterns

- **Zod env validation** — fails fast on boot if config is invalid
- **PostgreSQL pool** — `pg` with connection check on startup
- **SQL migrations** — ordered files in `db/migrations/`, tracked in `_migrations`
- **Winston logging** — console + daily rotate file logs
- **ApiError** — structured HTTP errors with global handler
- **Request logging** — method, path, status, duration on every request
- **CORS** — configured via `FE_ORIGIN`
- **Graceful shutdown** — SIGINT/SIGTERM close the server and DB pool cleanly
