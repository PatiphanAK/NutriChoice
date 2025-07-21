use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub price: Option<f64>,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64) -> Self {
        Product { id, name, price }
    }
}