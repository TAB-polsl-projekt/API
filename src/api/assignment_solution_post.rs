use chrono::Utc;
use diesel::dsl::{count_star, exists};
use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::post;
use rocket::serde::uuid::Uuid;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use crate::{define_api_response, define_response_data};
use crate::session::Session;

use crate::dbmodels::{Solution, User, UserRole, UserSolution};
use crate::schema::{assignments, roles, solutions, subject_role, user_role, user_solution, users};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: post_assignment_solution]
}

define_api_response!(enum PostAssignmetSolutionResponse {
    Ok => (200, "Solution has been submitted", (), ()),
});

define_api_response!(enum PostAssignmetSolutionError {
    BadRequest => (400, "Coauthors are not in the same role as user", (), ()),
    Forbidden => (403, "User does not belong to the subject", (), ()),
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(struct PostAssignmetSolutionRequestData {
    pub solution: Solution,
    pub coauthors_user_ids: Vec<String>
});

#[openapi(tag = "Assignments", operation_id = "postAssignmetSolution")]
#[post("/assignments/<assignment_id>/solution", data = "<request_data>")]
async fn post_assignment_solution(
    assignment_id: String,
    request_data: Json<PostAssignmetSolutionRequestData>,
    conn: crate::db::DbConn,
    session: Session
) -> Result<PostAssignmetSolutionResponse, PostAssignmetSolutionError> {
    let request_data = request_data.0;

    let mut sln = request_data.solution;
    let user_id = session.user_id;
    let coauthor_ids = request_data.coauthors_user_ids;

    conn.run(move |c| -> Result<_, PostAssignmetSolutionError> {

        let user: User = users::table.find(user_id.clone()).first(c)?;

        let user_assignment_role_ids: Vec<String> = UserRole::belonging_to(&user)
            .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
            .inner_join(subject_role::table.on(subject_role::role_id.eq(user_role::role_id)))
            .inner_join(assignments::table.on(assignments::subject_id.eq(subject_role::subject_id)))
            .filter(assignments::assignment_id.eq(&assignment_id))
            .select(roles::role_id)
            .distinct()
            .load(c)?;

        let user_has_access_to_subject: bool = !user_assignment_role_ids.is_empty();

        if !user_has_access_to_subject {
            return Err(PostAssignmetSolutionError::Forbidden(()));
        }

        let user_and_all_coauthors_have_common_role: bool = diesel::select(exists(
            user_role::table
                .filter(user_role::role_id.eq_any(&user_assignment_role_ids))
                .filter(user_role::user_id.eq_any(&coauthor_ids))
                .group_by(user_role::role_id)
                .having(count_star().eq(coauthor_ids.len() as i64)),
        ))
        .get_result(c)?;

        if !user_and_all_coauthors_have_common_role {
            return Err(PostAssignmetSolutionError::BadRequest(()))
        }

        let solution_id = Uuid::new_v4().to_string();
        sln.solution_id = solution_id.clone();
        sln.submission_date = Utc::now().naive_utc();
        sln.assignment_id = assignment_id;

        let _result = diesel::insert_into(solutions::table)
            .values(sln)
            .execute(c)?;

        let user_solutions: Vec<_> = coauthor_ids.into_iter()
            .chain(std::iter::once(user_id))
            .map(|user_id| {
                UserSolution { user_id, solution_id: solution_id.clone() }
            })
            .collect();

        let _result = diesel::insert_into(user_solution::table)
            .values(user_solutions)
            .execute(c)?;

        Ok(())
    })
    .await?;

    Ok(PostAssignmetSolutionResponse::Ok(()))
}