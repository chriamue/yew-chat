use super::MemoryMessageStorage;
use crate::api::{ReceiveResponse, SendRequest};
use crate::model::{ReceiveError, SendError};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use std::sync::Arc;

pub fn create_router(storage: Arc<MemoryMessageStorage>) -> Router {
    Router::new()
        .route("/send/:channel", post(send_message))
        .route("/receive/:channel", get(receive_messages))
        .with_state(storage)
}

#[utoipa::path(
    post,
    operation_id = "send",
    path = "/send/{channel}",
    request_body = SendRequest,
    responses(
        (status = 200, description = "Message sent successfully", body = ()),
        (status = 400, description = "Invalid request data"),
    )
)]
pub async fn send_message(
    State(storage): State<Arc<MemoryMessageStorage>>,
    Path(channel): Path<String>,
    Json(request): Json<SendRequest>,
) -> Result<Json<()>, Json<SendError>> {
    storage
        .send_message(&channel, request.message)
        .await
        .map(|_| Json(()))
        .map_err(Json)
}

#[utoipa::path(
    get,
    operation_id = "receive",
    path = "/receive/{channel}",
    responses(
        (status = 200, description = "Messages received successfully", body = ReceiveResponse),
        (status = 400, description = "Invalid request data"),
    )
)]
pub async fn receive_messages(
    State(storage): State<Arc<MemoryMessageStorage>>,
    Path(channel): Path<String>,
) -> Result<Json<ReceiveResponse>, Json<ReceiveError>> {
    storage
        .receive_messages(&channel)
        .await
        .map(|messages| Json(ReceiveResponse { messages }))
        .map_err(Json)
}
