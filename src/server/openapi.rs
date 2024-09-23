use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::routes::receive_messages,
        super::routes::send_message,
    ),
    components(
        schemas(
            crate::api::SendRequest,
            crate::api::ReceiveResponse,
            crate::model::Message,
        )
    ),
    tags(
        (name = "API", description = "Chat API")
    )
)]
pub struct ApiDoc;
