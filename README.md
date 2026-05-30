# My Templates

Personal collection of project scaffolds I reach for when starting new work. Each template is a self-contained folder you can copy out and rename — not a monorepo or shared library.

## Templates

| Template                                       | Stack                        | Description                                                                                                                                    |
| ---------------------------------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [`rust_be_template/`](rust_be_template/)       | Rust, Axum, SQLx, PostgreSQL | Production-style HTTP API: multi-crate workspace, Bearer API key auth, OpenAPI/Swagger, structured logging, Docker Compose for local Postgres. |
| [`express_be_template/`](express_be_template/) | Node.js, Express, TypeScript | Minimal API starter with Zod env validation, Winston logging, route/service layout, and graceful shutdown.                                     |

See each template's README for prerequisites, quick start, and project structure.

## Usage

Copy a template into a new project directory and follow its README:

```bash
cp -r rust_be_template ~/my-new-api
# or
cp -R express_be_template ~/my-new-api
```
