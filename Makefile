include .env
export $(shell sed 's/=.*//' .env)

# Start the database
dbup:
	@echo "🐘 Starting PostgreSQL container..."
	cd database/postgres && docker compose up -d

# Run everything: env + dbup
build: dbup
	@echo "🚀 Database is up and running."

