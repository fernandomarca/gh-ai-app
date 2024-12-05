use http::routes::app_routes;
use tokio::net::TcpListener;

mod config;
mod http;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("Server running on port {}", listener.local_addr().unwrap());
    axum::serve(listener, app_routes()).await.unwrap();
}
