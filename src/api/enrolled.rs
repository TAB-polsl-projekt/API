use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::{Login, User};
use crate::{define_api_response, define_response_data};
use crate::schema::{logins, users};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Ok", Vec<GetUsersResponseData>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(pub struct GetUsersResponseData {
    pub user_id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub student_id: Option<String>,
    pub is_admin: bool,
});

// Get all users
#[openapi(tag = "Users", operation_id = "getUsers")]
#[get("/users")]
pub async fn endpoint(conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let users_and_logins: Vec<(User, Login)> = conn.run(move |c| {
        users::table
            .inner_join(logins::table.on(logins::user_id.eq(users::user_id)))
            .load(c)
    }).await?;

    let result = users_and_logins.into_iter()
        .map(|x| {
            let user = x.0;
            let login = x.1;

            GetUsersResponseData {
                user_id: user.user_id,
                name: user.name,
                surname: user.surname,
                email: login.email,
                student_id: user.student_id,
                is_admin: user.is_admin
            }
        })
        .collect();

    Ok(Response::Ok(result))
}