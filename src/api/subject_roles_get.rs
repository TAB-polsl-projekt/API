use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{admin_session::AdminSession, dbmodels::Role, define_api_response, define_response_data, schema::{roles, subject_role}};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Returns all roles assigned to subject", ResponseData, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(
    pub struct ResponseData {
        pub roles: Vec<Role>
    }
);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LoginData {
    pub email: String,
    pub password_hash: String,
}

/// Get all roles assigned to subject
#[openapi(tag = "Subjects", operation_id = "getSubjectRoles")]
#[get("/subjects/<subject_id>/roles")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {

    let roles= conn.run(|c| -> Result<_, Error> {
        let roles = subject_role::table
            .inner_join(roles::table.on(roles::role_id.eq(subject_role::role_id)))
            .filter(subject_role::subject_id.eq(subject_id))
            .select(roles::all_columns)
            .load(c)?;

        Ok(roles)
    }).await?;

    Ok(Response::Ok(ResponseData { roles }))
}