use crate::entities::product::{Product, NewProduct, UpdateProduct};
use std::sync::Arc;
use tokio::sync::Mutex;

type ProductStore = Arc<Mutex<Vec<Product>>>;

lazy_static::lazy_static! {
    static ref PRODUCTS_DB: ProductStore = Arc::new(Mutex::new(vec![
        Product::new(1, "มาม่าคับ".to_string(), 1200.00),
        Product::new(2, "ไวตามิลล์".to_string(), 25.00),
        Product::new(3, "แลคตาซอย".to_string(), 75.00),
    ]));
}

pub async fn get_all_products() -> Vec<Product> {
    let db = PRODUCTS_DB.lock().await;
    db.clone()
}

pub async fn get_product_by_id(id: u32) -> Option<Product> {
    let db = PRODUCTS_DB.lock().await;
    db.iter().find(|p| p.id == id).cloned()
}

pub async fn create_product(new_product: NewProduct) -> Product {
    let mut db = PRODUCTS_DB.lock().await;
    let next_id = db.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let product = Product::new(next_id, new_product.name, new_product.price);
    db.push(product.clone());
    product
}


pub async fn update_product(id: u32, update_data: UpdateProduct) -> Option<Product> {
    let mut db = PRODUCTS_DB.lock().await;
    if let Some(product) = db.iter_mut().find(|p| p.id == id) {
        if let Some(name) = update_data.name {
            product.name = name;
        }
        if let Some(price) = update_data.price {
            product.price = price;
        }
        return Some(product.clone());
    }
    None
}

pub async fn delete_product(id: u32) -> bool {
    let mut db = PRODUCTS_DB.lock().await;
    let initial_len = db.len();
    db.retain(|p| p.id != id);
    db.len() < initial_len
}