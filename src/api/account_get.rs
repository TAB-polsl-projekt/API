use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;

use crate::dbmodels::User;
use crate::dbschema::users;
use crate::session::Session;

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
pub async fn endpoint(conn: crate::db::DbConn, session: Session) -> Result<Json<Response>, BadRequest<Json<Error>>> {
    let user_id = session.user_id;

    conn.run(move |c| -> Result<_, Error> {

        let result: User = {
            users::table
                .filter(users::user_id.eq(user_id))
                .select(users::all_columns)
                .first(c)
                .map_err(|_e| Error::Other("Failed to find the user".to_string()))?
        };

        Ok(Json(result))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}