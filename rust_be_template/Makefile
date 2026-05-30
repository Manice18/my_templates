.PHONY: start stop dev migrate migrate-revert migrate-info migrate-new status

require-postgres:
	@set -a && [ -f .env ] && . ./.env && set +a; \
	if [ -z "$$DATABASE_URL" ]; then \
		echo "ERROR: DATABASE_URL is not set (check .env)"; \
		exit 1; \
	fi; \
	if command -v psql >/dev/null 2>&1; then \
		if ! psql "$$DATABASE_URL" -c "SELECT 1" >/dev/null 2>&1; then \
			echo "ERROR: Cannot connect to PostgreSQL at DATABASE_URL"; \
			echo "       Run: make services-start"; \
			exit 1; \
		fi; \
	else \
		echo "WARNING: psql not found; skipping Postgres connectivity check"; \
	fi

services-start:
	@echo "Starting PostgreSQL..."
	@docker compose up -d postgres
	@echo "Waiting for PostgreSQL to be healthy..."
	@sleep 5
	@docker compose ps postgres

services-stop:
	@echo "Stopping PostgreSQL..."
	@docker compose stop postgres

start: services-start migrate
	@echo "Starting API server..."
	@set -a && [ -f .env ] && . ./.env && set +a && \
		RUST_LOG=info cargo run -p api

stop: services-stop

dev: services-start migrate require-postgres
	@echo "Starting API in development mode..."
	@set -a && [ -f .env ] && . ./.env && set +a && \
		RUST_LOG=info cargo run -p api

migrate:
	@set -a && [ -f .env ] && . ./.env && set +a && \
		sqlx migrate run

migrate-revert:
	@set -a && [ -f .env ] && . ./.env && set +a && \
		sqlx migrate revert

migrate-info:
	@set -a && [ -f .env ] && . ./.env && set +a && \
		sqlx migrate info

migrate-new:
	@if [ -z "$(NAME)" ]; then \
		echo "Usage: make migrate-new NAME='description_of_change'"; \
		exit 1; \
	fi
	@sqlx migrate add $(NAME)

status:
	@echo "=== Service Status ==="
	@docker compose ps postgres 2>/dev/null || echo "Docker Compose not available"
