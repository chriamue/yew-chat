use super::MessageStorage;
use crate::model::{ReceiveError, SendError};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::api::{ReceiveResponse, SendRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_router(storage: Arc<Mutex<dyn MessageStorage>>) -> Router {
    Router::new()
        .route("/send/:channel", post(send_message))
        .route("/receive/:channel", get(receive_messages))
        .with_state(storage)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    operation_id = "send",
    tag = "send",
    path = "/send/{channel}",
    request_body = SendRequest,
    responses(
        (status = 200, description = "Message sent successfully", body = ()),
        (status = 400, description = "Invalid request data"),
    )
)]
pub async fn send_message(
    State(sender): State<Arc<Mutex<dyn MessageStorage>>>,
    Path(channel): Path<String>,
    Json(message): Json<SendRequest>,
) -> Result<Json<()>, Json<SendError>> {
    let message = message.message;
    sender
        .lock()
        .await
        .send_message(&channel, message)
        .await
        .map(|_| Json(()))
        .map_err(Json)
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    operation_id = "receive",
    tag = "receive",
    path = "/receive/{channel}",
    responses(
        (status = 200, description = "Messages received successfully", body = ReceiveResponse),
        (status = 400, description = "Invalid request data"),
    )
)]
pub async fn receive_messages(
    State(receiver): State<Arc<Mutex<dyn MessageStorage>>>,
    Path(channel): Path<String>,
) -> Result<Json<ReceiveResponse>, Json<ReceiveError>> {
    receiver
        .lock()
        .await
        .receive_messages(&channel)
        .await
        .map(|messages| Json(ReceiveResponse { messages }))
        .map_err(Json)
}
