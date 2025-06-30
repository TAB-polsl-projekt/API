use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbmodels::User;
use crate::session::Session;
use crate::schema::{roles, subjects, user_role, users};
use crate::{define_api_response, define_response_data};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_subject_teachers]
}

define_api_response!(enum GetSubjectTeachersResponse {
    Ok => (200, "Ok", Vec<GetSubjectTeachersResponseData>, ()),
});

define_api_response!(enum GetSubjectTeachersError {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(struct GetSubjectTeachersResponseData {
    pub full_name: String,
    pub user_id: String
});

/// Get subject's teachers
#[openapi(tag = "Subjects", operation_id = "getSubjectTeachers")]
#[get("/subject/<subject_id>/teachers")]
async fn get_subject_teachers(subject_id: String, conn: crate::db::DbConn, _session: Session) -> Result<GetSubjectTeachersResponse, GetSubjectTeachersError> {
    let teachers: Vec<User> = conn.run(move |c| {
        subjects::table
            .inner_join(roles::table.on(subjects::editor_role_id.eq(roles::role_id.nullable())))
            .inner_join(user_role::table.on(user_role::role_id.eq(roles::role_id)))
            .inner_join(users::table.on(users::user_id.eq(user_role::user_id)))
            .filter(subjects::subject_id.eq(subject_id))
            .select(users::all_columns)
            .distinct()
            .get_results(c)
    }).await.unwrap();

    let result = teachers.into_iter()
        .map(|x| {
            let full_name = format!("{} {}", x.name, x.surname);
            let user_id = x.user_id;

            GetSubjectTeachersResponseData {
                full_name,
                user_id
            }
        })
        .collect();

    Ok(GetSubjectTeachersResponse::Ok(result))
}