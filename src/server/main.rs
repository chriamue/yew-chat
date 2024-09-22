use std::sync::Arc;
use tower_http::cors::CorsLayer;
use yew_chat::server::create_router;
use yew_chat::server::MemoryMessageStorage;

#[tokio::main]
async fn main() {
    let storage = Arc::new(MemoryMessageStorage::new());

    // Create the router
    let app = create_router(storage).layer(CorsLayer::permissive());

    // Start the Axum server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Starting server at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
