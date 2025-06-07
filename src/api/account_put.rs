use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;

use crate::session::Session;
use crate::{dbmodels::UserUpdate, dbschema::users};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
}

#[openapi(tag = "Account")]
#[put("/account", data = "<user>")]
pub async fn endpoint(user: Json<UserUpdate>, conn: crate::db::DbConn, session: Session) -> Result<(), BadRequest<Json<Error>>> {
    let user_id = session.user_id;
    let update_user = user.0;
    
    conn.run(move |c| -> Result<_, Error> {

        let _rows_affected = diesel::update(users::table.filter(users::user_id.eq(user_id)))
            .set(&update_user)
            .execute(c)
            .map_err(|_e| Error::Other("Failed to update the record".to_string()))?;

        Ok(())
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}