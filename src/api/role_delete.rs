use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::delete;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::{admin_session::AdminSession, define_api_response, schema::roles};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Role has been deleted", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

/// Delete a role
#[openapi(tag = "Roles", operation_id = "deleteRole")]
#[delete("/roles/<role_id>")]
pub async fn endpoint(role_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let _result = conn.run(move |c| {
        diesel::delete(roles::table)
            .filter(roles::role_id.eq(role_id))
            .execute(c)
    }).await?;

    Ok(Response::Ok(()))
}