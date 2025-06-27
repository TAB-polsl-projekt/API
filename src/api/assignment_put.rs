use diesel::dsl::{exists, select};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, update, AsChangeset};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;

use crate::schema::{assignments, subject_role, user_role};
use crate::session::Session;

/// Fields for updating an assignment. All fields are optional; only provided ones will be changed.
#[derive(AsChangeset, Deserialize, schemars::JsonSchema, Debug)]
#[diesel(table_name = assignments)]
pub struct AssignmentUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub accepted_mime_types: Option<String>,
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
}

#[openapi(tag = "Account")]
#[put("/assignments/<assignment_id>", data = "<assignment_update>")]
pub async fn endpoint(
    assignment_id: String,
    assignment_update: Json<AssignmentUpdate>,
    conn: crate::db::DbConn,
    session: Session
) -> Result<(), BadRequest<Json<Error>>> {
    let assignment_update = assignment_update.0;
    let user_id = session.user_id.clone();

    conn.run(move |c| -> Result<_, Error> {
        // Check if the user has the 'editor' role for the assignment's subject
        let editor_check = assignments::table
            .inner_join(
                subject_role::table.on(subject_role::subject_id.eq(assignments::subject_id))
            )
            .inner_join(
                user_role::table.on(user_role::role_id.eq(subject_role::role_id))
            )
            .filter(user_role::user_id.eq(user_id.clone()))
            .filter(assignments::assignment_id.eq(&assignment_id));

        let is_editor: bool = select(exists(editor_check))
            .get_result(c)
            .map_err(|_| Error::Other("Failed to verify editor role".into()))?;

        if !is_editor {
            return Err(Error::Other("User is not editor for this subject".into()));
        }

        let rows = update(
            assignments::table.filter(assignments::assignment_id.eq(&assignment_id))
        )
        .set(&assignment_update)
        .execute(c)
        .map_err(|_| Error::Other("Failed to update assignment".into()))?;

        if rows == 0 {
            return Err(Error::Other("Assignment not found".into()));
        }

        Ok(())
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}
