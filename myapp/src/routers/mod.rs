pub mod product_routes;

use axum::{Router, routing::get};

pub fn app_router() -> Router {
    Router::new()
        .merge(product_routes::routes())
        .route("/", get(root_handler))
}

async fn root_handler() -> &'static str {
    "Welcome to the API! Visit /products for data."
}
