use crate::api::{ReceiveResponse, SendRequest};
use crate::handler::MessageHandler;
use crate::model::{Message, ReceiveError, SendError};
use gloo::net::http::Request;
use log::error;

#[derive(Clone, PartialEq)]
pub struct RequestMessageHandler {
    pub host: String,
}

impl MessageHandler for RequestMessageHandler {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError> {
        let body = SendRequest { message };
        let url = format!("{}/send/{}", self.host, channel);
        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).map_err(|e| {
                error!("Failed to serialize: {}", e);
                SendError::UnknownError
            })?)
            .map_err(|e| {
                error!("Failed to build request: {}", e);
                SendError::UnknownError
            })?
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send: {}", e);
                SendError::UnknownError
            })?;

        if response.status() == 200 {
            Ok(())
        } else {
            error!("Error response: {}", response.status());
            Err(SendError::UnknownError)
        }
    }

    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let url = format!("{}/receive/{}", self.host, channel);
        let response = Request::get(&url).send().await.map_err(|e| {
            error!("Failed to send: {}", e);
            ReceiveError::UnknownError
        })?;

        if response.status() == 200 {
            response
                .json::<ReceiveResponse>()
                .await
                .map(|r| r.messages)
                .map_err(|e| {
                    error!("Failed to parse: {}", e);
                    ReceiveError::UnknownError
                })
        } else {
            error!("Error response: {}", response.status());
            Err(ReceiveError::UnknownError)
        }
    }
}
