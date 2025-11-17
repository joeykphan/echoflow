
.PHONY: help dev test lint fmt clean migrate-up migrate-down db-reset docker-up docker-down

help:
	@echo "Available commands:"
	@echo "  make dev          - Run development server with auto-reload"
	@echo "  make test         - Run all tests"
	@echo "  make lint         - Run clippy linter"
	@echo "  make fmt          - Format code"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make migrate-up   - Run database migrations"
	@echo "  make migrate-down - Rollback last migration"
	@echo "  make db-reset     - Reset database (drop and recreate)"
	@echo "  make docker-up    - Start Docker containers"
	@echo "  make docker-down  - Stop Docker containers"

dev:
	cargo watch -x run

test:
	DATABASE_TEST_URL=postgresql://postgres:password@localhost:5432/financeapp_test cargo test -- --test-threads=1

test-verbose:
	DATABASE_TEST_URL=postgresql://postgres:password@localhost:5432/financeapp_test cargo test -- --nocapture --test-threads=1

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

clean:
	cargo clean

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

db-reset:
	@echo "Dropping and recreating databases..."
	psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS financeapp;"
	psql -h localhost -U postgres -c "CREATE DATABASE financeapp;"
	psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS financeapp_test;"
	psql -h localhost -U postgres -c "CREATE DATABASE financeapp_test;"
	sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/financeapp
	sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/financeapp_test
	@echo "Databases reset successfully!"

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down
