CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    calories INTEGER,
    sugar INTEGER,
    sodium INTEGER,
    protein INTEGER,
    cholesterol INTEGER,
    total_fat INTEGER,
    saturated_fat INTEGER,
    total_carbohydrate INTEGER,
    dietary_fiber INTEGER
);

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    product_id UUID NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);