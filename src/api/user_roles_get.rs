use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::Serialize;

use crate::dbmodels::Role;
use crate::define_api_response;
use crate::schema::{roles, user_role};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", ResponseData, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "This user cannot view other user's roles", (), ()),
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[derive(Debug, Serialize, JsonSchema)]
pub struct ResponseData {
    pub roles: Vec<Role>
}

#[openapi(tag = "Users", operation_id = "getUserRoles")]
#[get("/users/<user_id>/roles")]
pub async fn endpoint(user_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    if user_id != session.user_id && !session.is_admin {
        return Err(Error::Unauthorized(()));
    }

    let result = conn.run(move |c| {
        user_role::table
            .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
            .filter(user_role::user_id.eq(user_id))
            .select(roles::all_columns)
            .load(c)
    }).await?;

    Ok(Response::Ok(ResponseData { roles: result }))
}