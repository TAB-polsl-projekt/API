use diesel::RunQueryDsl;
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{admin_session::AdminSession, dbmodels::UserRole, define_api_response, schema::user_role};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Returns session id", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RoleData {
    pub role_id: String,
}

/// Test
#[openapi(tag = "Users", operation_id = "postUserRole")]
#[post("/user/<user_id>/role", data = "<role_id>")]
pub async fn endpoint(user_id: String, role_id: Json<RoleData>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let role_id = role_id.role_id.clone();
    
    let _ = conn.run(|c| {
        let user_role = UserRole { role_id, user_id };

        diesel::insert_into(user_role::table)
            .values(user_role)
            .execute(c)
    }).await?;

    Ok(Response::Ok(()))
}