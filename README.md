# NutriChoice

**NutriChoice** is a lightweight backend API built with Go and PostgreSQL, designed to help users track, manage, and explore nutritional products. The goal is to provide a fast and reliable foundation for food and nutrition-based applications, supporting core features such as product lookup, nutritional values, and personalized suggestions.

## 🚀 Features

- RESTful API for product and nutrition management
- PostgreSQL database for reliable and persistent storage
- Containerized using Docker and Docker Compose for easy development and deployment
- Clean architecture for future extension (e.g., user auth, recommendations, image-based product input)

## 🛠️ Tech Stack

- **Go** – backend server and API logic (currently using fiber)
- **PostgreSQL** – relational database for product data
- **Docker** – containerized services
- **Makefile** – simple command runner for common tasks

## 🛠️ Setup

1. **Environment Variables**  
   Configure environment variables by editing the `set_env.sh` script:
   ```bash
   # Inside set_env.sh example
    export POSTGRES_USER="your_postgres_user"
    export POSTGRES_PASSWORD="your_strong_password"
    export POSTGRES_DB="products_db"```
