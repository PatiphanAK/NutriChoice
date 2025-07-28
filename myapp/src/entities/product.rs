use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub calories: f64,
    pub suger: f64,
    pub sodium: f64,
    pub protein: f64,
    pub cholesterol: f64,
    pub total_fat: f64,
    pub saturated_fat: f64,
    pub total_carbohydrate: f64,
    pub dietary_fiber: f64,
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
        Product {
            id,
            name,
            price,
            ..Default::default()
        }
    }
}