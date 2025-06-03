use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::CookieJar, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::User;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

pub type Response = User;

#[openapi(tag = "Account")]
#[get("/account")]
pub async fn endpoint(conn: crate::db::DbConn, jar: &CookieJar<'_>) -> Result<Json<Response>, BadRequest<Json<Error>>> {
    let session_id = jar.get("session_id").map(|cookie| cookie.value())
            .ok_or(Error::Other("Invalid session ID".to_string())).map_err(|e| BadRequest(Json(e)))?.to_string();

    conn.run(move |c| -> Result<_, Error> {

        let user_id: String = {
            use crate::dbschema::session_refresh_keys::dsl::*;
            
            session_refresh_keys
                .filter(refresh_key_id.eq(session_id))
                .select(user_id)
                .first(c)
                .map_err(|_e| Error::Other("Invalid session ID".to_string()))?
        };

        let result: User = {
            let uid = user_id;
            use crate::dbschema::users::dsl::*;

            users
                .filter(user_id.eq(uid))
                .select(users::all_columns())
                .first(c)
                .map_err(|_e| Error::Other("Failed to find the user".to_string()))?
        };

        Ok(Json(result))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}