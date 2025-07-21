mod entities;
mod routers;
mod services;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {:?}", addr);
    axum::serve(listener, routers::app_router()).await.unwrap();
}
