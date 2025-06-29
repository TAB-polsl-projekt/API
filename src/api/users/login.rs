use diesel::{AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_api_response;
use crate::schema::logins;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: put_user_login]
}

define_api_response!(pub enum PutLoginResponse {
    Ok => (200, "Login info updated", (), ())
});

define_api_response!(pub enum PutLoginError {
    Forbidden => (403, "User cannot modify other users", (), ()),
    InternalServerError => (500, "Failed to update user account", (), (diesel::result::Error)),
});

#[derive(Debug, Deserialize, Serialize, AsChangeset, JsonSchema)]
#[diesel(table_name = logins)]
pub struct LoginUpdate {
    pub email: Option<String>,
    pub passwd_hash: Option<String>
}

#[openapi(tag = "Users")]
#[put("/users/<user_id>/login", data = "<login_update>")]
pub async fn put_user_login(user_id: String, login_update: Json<LoginUpdate>, conn: crate::db::DbConn, session: Session) -> Result<PutLoginResponse, PutLoginError> {
    let login_update = login_update.0;

    if user_id != session.user_id && !session.is_admin {
        return Err(PutLoginError::Forbidden(()));
    }

    conn.run(move |c| {
        diesel::update(logins::table.filter(logins::user_id.eq(user_id)))
            .set(&login_update)
            .execute(c)
    }).await?;

    Ok(PutLoginResponse::Ok(()))
}