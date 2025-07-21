use axum::{
    routing::{get, post, patch, delete},
    Json,
    extract::Path,
    http::StatusCode,
    Router,
};
use crate::entities::product::{Product, NewProduct, UpdateProduct};
use crate::services::product_services;

pub fn routes() -> Router {
    Router::new()
        .route("/products", get(get_all_products_handler))
        .route("/products", post(create_product_handler))
        .route("/products/{id}", get(get_product_by_id_handler))
        .route("/products/{id}", patch(update_product_handler))
        .route("/products/{id}", delete(delete_product_handler))
}


// === Handlers for Products ===

async fn get_all_products_handler() -> Json<Vec<Product>> {
    let products = product_services::get_all_products().await;
    Json(products)
}

async fn get_product_by_id_handler(Path(id): Path<u32>) -> Result<Json<Product>, StatusCode> {
    if let Some(product) = product_services::get_product_by_id(id).await {
        Ok(Json(product))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_product_handler(Json(payload): Json<NewProduct>) -> (StatusCode, Json<Product>) {
    let product = product_services::create_product(payload).await;
    (StatusCode::CREATED, Json(product))
}

async fn update_product_handler(
    Path(id): Path<u32>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    if let Some(product) = product_services::update_product(id, payload).await {
        Ok(Json(product))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_product_handler(Path(id): Path<u32>) -> StatusCode {
    if product_services::delete_product(id).await {
        StatusCode::NO_CONTENT // 204 No Content for successful deletion
    } else {
        StatusCode::NOT_FOUND
    }
}