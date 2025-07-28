# NutriChoice

**NutriChoice** is a lightweight backend API built with Rust and PostgreSQL, designed to help users track, manage, and explore nutritional products. The goal is to provide a fast and reliable foundation for food and nutrition-based applications, supporting core features such as product lookup, nutritional values, and personalized suggestions.

## Features

- RESTful API for product and nutrition management
- PostgreSQL database for reliable and persistent storage
- Containerized using Docker and Docker Compose for easy development and deployment
- Clean architecture for future extension (e.g., user auth, recommendations, image-based product input)

## Tech Stack

- **Rust** – backend server and API logic 
- **PostgreSQL** – relational database for product data
- **Docker** – containerized services
- **Makefile** – simple command runner for common tasks
## Environment Variables

Here's an example of a `.env` file you can use for development:

```env
POSTGRES_USER=your_postgres_user
POSTGRES_PASSWORD=your_strong_password
POSTGRES_DB=products_db
```

