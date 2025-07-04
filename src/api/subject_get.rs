use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

use crate::dbmodels::{Assignment, Role, Subject, SubjectRole, User, UserRole};
use crate::schema::{subject_role, user_role};
use crate::schema::{roles, subjects, users};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Debug, schemars::JsonSchema, Serialize, Deserialize)]
struct AllAssignmentData {
    #[serde(flatten)]
    assignment_data: Assignment,
    attendance: bool
}

#[derive(Debug, schemars::JsonSchema, Serialize, Deserialize)]
struct ResponseData {
    subject_name: String,
    editor_role_id: Option<String>,
    assignments: Vec<AllAssignmentData>,
}

define_api_response!(enum Response {
    Ok => (200, "Ok", ResponseData, ())
});

define_api_response!(enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error))
});

#[openapi(tag = "Subjects", operation_id = "getSubject")]
#[get("/subjects/<subject_id>")]
async fn endpoint(subject_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;
    let is_admin = session.is_admin;

    conn.run(move |c| {

        let result = {
            let subject = if is_admin {
                subjects::table.find(subject_id).first(c)?
            } else {
                let user: User = users::table.find(user_id).first(c)?;

                let roles: Vec<Role> = UserRole::belonging_to(&user)
                    .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
                    .select(roles::all_columns)
                    .load(c)?;

                let subject: Subject = SubjectRole::belonging_to(&roles)
                    .inner_join(subjects::table.on(subjects::subject_id.eq(subject_role::subject_id)))
                    .filter(subjects::subject_id.eq(subject_id))
                    .select(subjects::all_columns)
                    .first(c)?;

                subject
            };

            let assignments = Assignment::belonging_to(&subject).load(c)?;
            let subject_name = subject.subject_name;
            let editor_role_id = subject.editor_role_id;

            let assignments = assignments.into_iter()
                .map(|assignment| {
                    AllAssignmentData {
                        assignment_data: assignment,
                        attendance: true
                    }
                })
                .collect();

            ResponseData {
                subject_name,
                editor_role_id,
                assignments
            }
        };

        Ok(Response::Ok(result))
    }).await
}
