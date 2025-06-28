use diesel::dsl::exists;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::Serialize;

use crate::dbmodels::User;
use crate::define_api_response;
use crate::schema::{user_role, users};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Vec<ResponseData>, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "This user cannot view users in other roles", (), ()),
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[derive(Debug, Serialize, JsonSchema)]
pub struct ResponseData {
    pub user_id: String,
    pub full_name: String,
    pub student_id: String,
}

#[openapi(tag = "Roles", operation_id = "getRoleUsers")]
#[get("/roles/<role_id>/users")]
pub async fn endpoint(role_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let result: Vec<User> = conn.run(move |c| {
        if !session.is_admin {
            let can_view_role: bool = diesel::select(exists(
                user_role::table
                    .filter(user_role::user_id.eq(&session.user_id))
                    .filter(user_role::role_id.eq(&role_id))
            )).get_result(c)?;

            if !can_view_role {
                return Err(Error::Unauthorized(()));
            }
        }

        let users = user_role::table
            .inner_join(users::table.on(users::user_id.eq(user_role::user_id)))
            .filter(user_role::role_id.eq(&role_id))
            .filter(user_role::user_id.ne(&session.user_id))
            .select(users::all_columns)
            .load(c)?;

        Ok(users)
    }).await?;

    let response_data = result.into_iter()
        .map(|x| {
            let user_id = x.user_id;
            let full_name = format!("{} {}", x.name, x.surname);
            let student_id = x.student_id.unwrap_or("".to_string());

            ResponseData { user_id, full_name, student_id }
        })
        .collect();

    Ok(Response::Ok(response_data))
}