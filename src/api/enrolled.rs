use diesel::RunQueryDsl;
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::User;
use crate::define_api_response;
use crate::schema::users;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Vec<User>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Account", operation_id = "getUsers")]
#[get("/users")]
pub async fn endpoint(conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        users::table
            .get_results(c)
    }).await?;

    Ok(Response::Ok(result))
}