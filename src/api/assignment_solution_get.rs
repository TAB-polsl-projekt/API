use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbmodels::{Solution};
use crate::schema::{solutions, user_solution};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Solution, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "getAssignmentSolution")]
#[get("/assignments/<assignment_id>/solution")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;

    let result = conn.run(move |c| {
        solutions::table
            .inner_join(user_solution::table.on(user_solution::solution_id.eq(solutions::solution_id)))
            .filter(solutions::assignment_id.eq(assignment_id))
            .filter(user_solution::user_id.eq(user_id))
            .select(solutions::all_columns)
            .first(c)
    }).await.unwrap();

    Ok(Response::Ok(result))
}