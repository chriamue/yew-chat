use crate::api::{ReceiveResponse, SendRequest};
use crate::model::{Message, MessageReceiver, MessageSender, ReceiveError, SendError};
use async_trait::async_trait;
use gloo::net::http::{Request, Response};
use log::{error, info};

pub struct RequestMessageHandler {
    pub host: String,
}

async fn handle_response(response: Response) -> Result<(), SendError> {
    if response.status() == 200 {
        Ok(())
    } else {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to get response text".to_string());
        error!("Error response: Status {}, Body: {}", status, text);
        Err(SendError::UnknownError)
    }
}

#[async_trait(?Send)]
impl MessageSender for RequestMessageHandler {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError> {
        let body: SendRequest = SendRequest { message };
        let url = format!("{}/send/{}", self.host, channel);

        info!("Sending message to channel: {}", channel);

        let request = Request::post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).map_err(|e| {
                error!("Failed to serialize request body: {}", e);
                SendError::UnknownError
            })?)
            .map_err(|e| {
                error!("Failed to create request: {}", e);
                SendError::UnknownError
            })?;

        let response = request.send().await.map_err(|e| {
            error!("Failed to send request: {}", e);
            SendError::UnknownError
        })?;

        handle_response(response).await
    }
}

#[async_trait(?Send)]
impl MessageReceiver for RequestMessageHandler {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let url = format!("{}/receive/{}", self.host, channel);

        info!("Receiving messages from channel: {}", channel);

        let request = Request::get(&url).send().await.map_err(|e| {
            error!("Failed to send request: {}", e);
            ReceiveError::UnknownError
        })?;

        if request.status() == 200 {
            request
                .json::<ReceiveResponse>()
                .await
                .map_err(|e| {
                    error!("Failed to parse response: {}", e);
                    ReceiveError::UnknownError
                })
                .map(|response| response.messages)
        } else {
            let status = request.status();
            let text = request
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get response text".to_string());
            error!("Error response: Status {}, Body: {}", status, text);
            Err(ReceiveError::UnknownError)
        }
    }
}
