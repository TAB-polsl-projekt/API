use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::{Solution};
use crate::schema::{assignments, solutions, subjects};
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Vec<Solution>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (401, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "getSubjectSolutions")]
#[get("/subject/<subject_id>/solutions")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        subjects::table
            .inner_join(assignments::table.on(assignments::subject_id.eq(subjects::subject_id)))
            .inner_join(solutions::table.on(solutions::assignment_id.eq(assignments::assignment_id)))
            .filter(subjects::subject_id.eq(subject_id))
            .select(solutions::all_columns)
            .get_results(c)
    }).await?;

    Ok(Response::Ok(result))
}