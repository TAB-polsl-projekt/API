use diesel::{AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::define_api_response;
use crate::schema::users;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: update_account]
}

define_api_response!(pub enum Error {
    InternalServerError => (500, "Failed to update user account", String, (diesel::result::Error)),
});

#[derive(Debug, Deserialize, Serialize, AsChangeset, JsonSchema)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub student_id: Option<String>,
}

#[openapi(tag = "Account", operation_id = "putAccountInfo")]
#[put("/account", data = "<user>")]
pub async fn update_account(user: Json<UserUpdate>, conn: crate::db::DbConn, session: Session) -> Result<(), Error> {
    let user_id = session.user_id;
    let update_user = user.0;

    conn.run(move |c| -> Result<_, Error> {
        diesel::update(users::table.filter(users::user_id.eq(user_id)))
            .set(&update_user)
            .execute(c)?;
        Ok(())
    }).await
}