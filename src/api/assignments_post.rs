use diesel::dsl::exists;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbmodels::Assignment;
use crate::dbschema::{assignments, roles, user_subjects};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", String, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User is not an admin", String, ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "postAssignment")]
#[post("/assignments", data = "<assignment>")]
pub async fn endpoint(assignment: Json<Assignment>, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;
    let assignment = assignment.0;

    let _result = conn.run(move |c| {
        let is_user_admin_query = roles::table
            .inner_join(user_subjects::table.on(roles::role_id.eq(user_subjects::role_id)))
            .filter(roles::role_id.eq("0"))
            .filter(user_subjects::user_id.eq(user_id));

        let is_user_admin: bool = diesel::select(exists(is_user_admin_query))
            .get_result(c)?;

        if !is_user_admin {
            return Err(Error::Unauthorized("".to_owned()));
        }
            
        let insertion_result = diesel::insert_into(assignments::table)
            .values(assignment)
            .execute(c)?;

        Ok(insertion_result)
    }).await?;

    Ok(Response::Ok("".to_owned()))
}