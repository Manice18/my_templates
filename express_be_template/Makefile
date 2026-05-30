.PHONY: services-start services-stop dev migrate

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

migrate: require-postgres
	@npm run db:migrate

dev: services-start migrate require-postgres
	@npm run dev
