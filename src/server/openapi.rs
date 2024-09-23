use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::routes::receive_messages,
        super::routes::send_message,
    ),
    components(
        schemas(
            super::routes::SendRequest,
            super::routes::ReceiveResponse,
            crate::model::Message,
        )
    ),
    tags(
        (name = "API", description = "Chat API")
    )
)]
pub struct ApiDoc;
