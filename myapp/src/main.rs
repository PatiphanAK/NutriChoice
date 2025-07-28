mod entities;
mod routers;
mod services;

use axum::Router;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000";
    
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;
    
    println!("Server running on {}", addr);

    let app: Router = routers::app_router()
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| format!("Server error: {}", e))?;
        
    Ok(())
}