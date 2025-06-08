use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbschema::{session_refresh_keys};
use crate::{define_api_response};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", String, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

/// Test
#[openapi(tag = "Auth", operation_id = "postAuth")]
#[post("/auth")]
pub async fn endpoint(conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    todo!()
}