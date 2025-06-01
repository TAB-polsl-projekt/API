use std::fmt::Debug;
use std::marker::PhantomData;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response};
use rocket_okapi::r#gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use schemars::JsonSchema;
use serde::Serialize;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::OpenApiError;

// Define a trait for endpoint-specific error responses.
pub trait EndpointResponse {
    fn responses(generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError>;
}

// Generic error type parameterized by a marker type E that implements EndpointError.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ApiResponse<E: EndpointResponse> {
    /// The title of the error message.
    pub err: String,
    /// The description of the error.
    pub msg: Option<String>,
    /// HTTP Status Code returned.
    #[serde(skip)]
    pub http_status_code: u16,
    // Marker to link endpoint-specific documentation.
    _marker: PhantomData<E>,
}

// Implement OpenApiResponderInner for the generic error type.
impl<E: EndpointResponse> OpenApiResponderInner for ApiResponse<E> {
    fn responses(generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        E::responses(generator)
    }
}

impl<E: EndpointResponse> std::fmt::Display for ApiResponse<E> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Error `{}`: {}",
            self.err,
            self.msg.as_deref().unwrap_or("<no message>")
        )
    }
}

impl<E: EndpointResponse + Debug> std::error::Error for ApiResponse<E> {}

impl<'r, E: EndpointResponse> Responder<'r, 'static> for ApiResponse<E> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.http_status_code))
            .ok()
    }
}