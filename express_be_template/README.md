# Express Backend Template

A minimal Node.js + Express + TypeScript starter. Copy this folder when scaffolding a new API.

## Layout

```
express_be_template/
├── src/
│   ├── server.ts          # Entry point, graceful shutdown
│   ├── app.ts             # Express app, middleware, error handler
│   ├── config/            # env (zod) + winston logger
│   ├── errors/            # ApiError
│   ├── middleware/        # Shared middleware
│   ├── routes/            # Route modules + index registrar
│   └── services/          # Business logic
├── .env.example
├── package.json
└── tsconfig.json
```

## Quick start

```bash
cp -R express_be_template my_project_be
cd my_project_be
npm install
cp .env.example .env
npm run dev
```

Server defaults to **http://localhost:8080**.

## Example endpoints

| Method | Path                | Description        |
| ------ | ------------------- | ------------------ |
| `GET`  | `/api/health`       | Health check       |
| `GET`  | `/api/examples`     | List example items |
| `GET`  | `/api/examples/:id` | Get one item       |

## Adding a new resource

1. Create a service in `src/services/`.
2. Add a route module in `src/routes/` using the `makeXRouter()` factory pattern.
3. Register it in `src/routes/index.ts`.

## Scripts

| Script          | Purpose                            |
| --------------- | ---------------------------------- |
| `npm run dev`   | tsx watch — hot reload             |
| `npm run build` | esbuild bundle to `dist/server.js` |
| `npm start`     | Run production build               |
| `npm run check` | TypeScript typecheck               |

## Included patterns

- **Zod env validation** — fails fast on boot if config is invalid
- **Winston logging** — console + daily rotate file logs
- **ApiError** — structured HTTP errors with global handler
- **Request logging** — method, path, status, duration on every request
- **CORS** — configured via `FE_ORIGIN`
- **Graceful shutdown** — SIGINT/SIGTERM close the server cleanly
