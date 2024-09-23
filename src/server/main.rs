use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use yew_chat::server::MemoryMessageStorage;
use yew_chat::server::{create_router, ApiDoc};

#[tokio::main]
async fn main() {
    let storage = Arc::new(Mutex::new(MemoryMessageStorage::new()));

    // Create the router
    let app = create_router(storage)
        .layer(CorsLayer::permissive())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

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
