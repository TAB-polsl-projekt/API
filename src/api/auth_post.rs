use chrono::DateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::{private::cookie::CookieBuilder, CookieJar}, post, serde::json::Json, time::{Duration, OffsetDateTime}};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dbmodels::{Login, SessionId}, define_api_response, define_response_data, schema::{logins, session_ids, users}};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Returns session id", ResponseData, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

define_response_data!(
    pub struct ResponseData {
        pub is_admin: bool
    }
);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LoginData {
    pub email: String,
    pub password_hash: String,
}

/// Test
#[openapi(tag = "Auth", operation_id = "postAuth")]
#[post("/auth", data = "<login_data>")]
pub async fn endpoint(login_data: Json<LoginData>, conn: crate::db::DbConn, cookie_jar: &CookieJar<'_>) -> Result<Response, Error> {
    let login_data = login_data.0;
    let email = login_data.email;
    let hased_password = login_data.password_hash;

    let (session_id, is_admin) = conn.run(|c| -> Result<_, Error> {
        let login_info: Login = logins::table
            .filter(logins::email.eq(email))
            .filter(logins::passwd_hash.eq(hased_password))
            .first(c)?;

        let user_id = &login_info.user_id;
        let refresh_key_id = Uuid::new_v4().to_string();
        let expiration_time =  DateTime::from_timestamp(0, 0).unwrap().naive_utc();

        let session_id = SessionId {
            refresh_key_id: refresh_key_id.clone(),
            user_id: user_id.clone(),
            expiration_time
        };

        let _ = diesel::insert_into(session_ids::table)
            .values(session_id)
            .execute(c)?;

        let is_admin: bool = users::table
            .find(user_id)
            .select(users::is_admin)
            .first(c)?;

        Ok((refresh_key_id, is_admin))
    }).await?;

    let expires = OffsetDateTime::now_utc() + Duration::days(365);
    let session_cookie = CookieBuilder::new("session_id", session_id)
        .expires(expires)
        .secure(true)
        .build();

    cookie_jar.add(session_cookie);
    Ok(Response::Ok(ResponseData { is_admin }))
}