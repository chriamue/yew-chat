use super::MessageStorage;
use crate::model::{Message, ReceiveError, SendError};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use std::sync::Arc;

pub fn create_router(storage: Arc<dyn MessageStorage + Send + Sync>) -> Router {
    Router::new()
        .route("/send/:channel", post(send_message))
        .route("/receive/:channel", get(receive_messages))
        .with_state(storage)
}

#[axum::debug_handler]
async fn send_message(
    State(sender): State<Arc<dyn MessageStorage + Send + Sync>>,
    Path(channel): Path<String>,
    Json(message): Json<Message>,
) -> Result<Json<()>, Json<SendError>> {
    sender
        .send_message(&channel, message)
        .await
        .map(|_| Json(()))
        .map_err(Json)
}

#[axum::debug_handler]
async fn receive_messages(
    State(receiver): State<Arc<dyn MessageStorage + Send + Sync>>,
    Path(channel): Path<String>,
) -> Result<Json<Vec<Message>>, Json<ReceiveError>> {
    receiver
        .receive_messages(&channel)
        .await
        .map(Json)
        .map_err(Json)
}
