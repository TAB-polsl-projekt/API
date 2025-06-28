use diesel::RunQueryDsl;
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{admin_session::AdminSession, dbmodels::{Login, User}, define_api_response, schema::{logins, users}};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "User has been created", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AccountData {
    pub email: String,
    pub password_hash: String,
    pub student_id: Option<String>,
    pub name: String,
    pub surname: String,
    pub is_admin: bool,
}

/// Create a new user account
#[openapi(tag = "Account", operation_id = "postAccount")]
#[post("/account", data = "<account_data>")]
pub async fn endpoint(account_data: Json<AccountData>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let account_data = account_data.0;
        let user_id = Uuid::new_v4().to_string();
        let login_id = Uuid::new_v4().to_string();

        let user = User {
            user_id: user_id.clone(),
            name: account_data.name,
            surname: account_data.surname,
            student_id: account_data.student_id,
            is_admin: account_data.is_admin
        };

        let login = Login {
            login_id,
            user_id,
            email: account_data.email,
            passwd_hash: account_data.password_hash
        };

    let _ = conn.run(move |c| -> Result<_, Error> {
        let _ = diesel::insert_into(users::table)
            .values(&user)
            .execute(c)?;

        let _ = diesel::insert_into(logins::table)
            .values(&login)
            .execute(c)?;

        Ok(())
    }).await?;

    Ok(Response::Ok(()))
}