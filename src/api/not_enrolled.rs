use diesel::dsl::{exists, not};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbmodels::{Assignment, User};
use crate::dbschema::{assignments, subjects, user_solution_assignments, user_subjects, users};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Vec<User>, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "TEST", (), ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Account")]
#[get("/subjects/<subject_id>/users/not-enrolled")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    
    let result = conn.run(move |c| {
        users::table
            .filter(not(
                exists(
                    user_subjects::table
                        .filter(user_subjects::user_id.eq(users::user_id))
                        .filter(user_subjects::subject_id.eq(subject_id))
                )
            ))
            .select(users::all_columns)
            .get_results::<User>(c)
    }).await?;

    Ok(Response::Ok(result))
}