use diesel::{RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::Solution;
use crate::define_api_response;
use crate::schema::solutions;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", Vec<Solution>, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User is not an admin", (), ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Solutions", operation_id = "getSolutions")]
#[get("/solutions")]
pub async fn endpoint(conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        solutions::table
            .get_results(c)
    }).await?;

    Ok(Response::Ok(result))
}