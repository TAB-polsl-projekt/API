use diesel::dsl::exists;
use diesel::{AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{define_api_response, define_response_data};
use crate::schema::logins;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: put_user_login]
}

define_api_response!(enum PutLoginResponse {
    Ok => (200, "Login info updated", (), ())
});

define_api_response!(enum PutLoginError {
    BadRequest => (400, "Old password is incorrect", (), ()),
    Forbidden => (403, "User cannot modify other users", (), ()),
    InternalServerError => (500, "Failed to update user account", (), (diesel::result::Error)),
});

#[derive(Debug, Deserialize, Serialize, AsChangeset, JsonSchema)]
#[diesel(table_name = logins)]
struct LoginUpdate {
    pub email: Option<String>,
    pub passwd_hash: Option<String>,
}

define_response_data!(
    struct PutUserLoginRequestData {
        #[serde(flatten)]
        pub login_update: LoginUpdate,
        pub old_passwd_hash: Option<String>
    }
);

#[openapi(tag = "Users")]
#[put("/users/<user_id>/login", data = "<login_update>")]
async fn put_user_login(user_id: String, login_update: Json<PutUserLoginRequestData>, conn: crate::db::DbConn, session: Session) -> Result<PutLoginResponse, PutLoginError> {
    let login_update = login_update.0;
    let old_passwd_hash = login_update.old_passwd_hash;
    let login_update = login_update.login_update;

    conn.run(move |c| {
        if !session.is_admin {
            if user_id != session.user_id {
                return Err(PutLoginError::Forbidden(()));
            }

            let old_passwd_hash = old_passwd_hash.ok_or(PutLoginError::BadRequest(()))?;
            let is_old_password_correct: bool = diesel::select(exists(
                logins::table
                    .filter(logins::user_id.eq(&user_id))
                    .filter(logins::passwd_hash.eq(&old_passwd_hash))
            )).get_result(c)?;

            if !is_old_password_correct {
                return Err(PutLoginError::BadRequest(()));
            }
        }
    
        diesel::update(logins::table.filter(logins::user_id.eq(user_id)))
            .set(&login_update)
            .execute(c)?;

        Ok(())
    }).await?;

    Ok(PutLoginResponse::Ok(()))
}