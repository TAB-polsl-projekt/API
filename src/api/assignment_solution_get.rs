use chrono::Duration;
use rocket::get;
use rocket_okapi::{okapi::{openapi3::{OpenApi, Responses}, Map}, openapi, openapi_get_routes_spec, settings::OpenApiSettings, JsonSchema};
use rocket_okapi::okapi::schemars;
use rocket_okapi::{r#gen::OpenApiGenerator, OpenApiError};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
use super::endpoint_response::{ApiResponse, EndpointResponse};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: assignment_solution]
}

pub struct StatusError;
pub struct StatusResponse;

impl EndpointResponse for StatusError {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                Job ID does not exist. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                Unexpected server error. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl EndpointResponse for StatusResponse {
    fn responses(generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse, MediaType};

        let mut responses = Map::new();
        responses.insert(
            "200".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                The job has done processing. \
                "
                .to_string(),
                content: {
                    let mut content = Map::new();
                    content.insert(
                        "application/json".to_string(),
                        MediaType {
                            schema: Some(generator.json_schema::<SuccessData>()),
                            ..Default::default()
                        },
                    );
                    content
                },
                ..Default::default()
            }),
        );
        responses.insert(
            "202".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                The job is in progress. \
                "
                .to_string(),
                content: {
                    let mut content = Map::new();
                    content.insert(
                        "application/json".to_string(),
                        MediaType {
                            schema: Some(generator.json_schema::<InProgressData>()),
                            ..Default::default()
                        },
                    );
                    content
                },
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub enum BackendError {
    #[serde(rename = "Invalid session ID")]
    InvalidSessionId,
    #[serde(rename = "Internal error")]
    InternalError,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    BackendError(BackendError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {

}

#[serde_as]
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuccessData {
    /// Indicates if the job completed successfully
    was_successful: bool,
    
    /// Optional value explaining what went wrong during processing
    job_error: Option<Error>,

    #[serde_as(as = "DurationSeconds<i64>")]
    #[schemars(with = "i64")]
    /// Total amount of time that the job took to finish
    processing_time_in_seconds: Duration,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    Success(SuccessData),
}

#[openapi(tag = "Assignments", operation_id = "getAssignmentSolution")]
#[get("/assignments/<assignment_id>/solution")]
pub fn assignment_solution(assignment_id: String) -> Result<ApiResponse<StatusResponse>, ApiResponse<StatusError>> {
    todo!()
}