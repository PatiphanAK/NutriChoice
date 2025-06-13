# Load environment variables from .env using the script
setenv:
	@echo "🔧 Setting environment variables..."
	@source ./set_env.sh

# Start the database (assumes docker-compose.yml is inside database/postgres)
dbup:
	@echo "🐘 Starting PostgreSQL container..."
	cd database/postgres && docker compose up -d

# Run everything: setenv + dbup
build:
	@echo "🚀 Initializing environment and database..."
	@$(MAKE) setenv
	@$(MAKE) dbup
