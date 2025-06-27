use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::Assignment;
use crate::schema::{assignments, subjects, user_subject};
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Returns user id and solution data", Vec<(String, Assignment)>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (401, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "aaaa")]
#[get("/subjects/<subject_id>/student-assignments")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        subjects::table
            .inner_join(assignments::table.on(assignments::subject_id.eq(subject_id)))
            .inner_join(user_subject::table.on(user_subject::subject_id.eq(subjects::subject_id)))
            .select((user_subject::user_id, assignments::all_columns))
            .get_results(c)
    }).await?;

    Ok(Response::Ok(result))
}