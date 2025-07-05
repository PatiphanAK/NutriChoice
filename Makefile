include .env
export $(shell sed 's/=.*//' .env)

dbreset:
	@echo "💥 Removing old PostgreSQL volume..."
	cd database/postgres && docker compose down -v

dbup:
	@echo "🐘 Starting PostgreSQL container..."
	cd database/postgres && docker compose up -d


# Run everything: env + dbup
build: dbup
	@echo "🚀 Database is up and running."

run_app_api :
	@echo "Starting the API server..."
	cd myapp && cargo run