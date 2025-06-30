use chrono::Utc;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use itertools::Itertools;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{post, routes};
use rocket::form::FromForm;
use rocket::serde::uuid::Uuid;
use crate::define_api_response;
use crate::session::Session;

use crate::dbmodels::{Solution, UserSolution};
use crate::schema::{assignments, solutions, subject_role, user_role, user_solution};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![post_assignment_solution]
}

define_api_response!(enum PostAssignmetSolutionResponse {
    Ok => (200, "Solution has been submitted", (), ()),
});

define_api_response!(enum PostAssignmetSolutionError {
    BadRequest => (400, "User or co-authors are not in the role", (), ()),
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error, std::io::Error)),
});

#[derive(FromForm)]
struct PostAssignmetSolutionRequestData<'r> {
    pub solution: Json<Solution>,
    pub role_id: String,
    pub coauthors_user_ids: Vec<String>,
    pub solution_file: TempFile<'r>
}

#[post("/assignments/<assignment_id>/solution", data = "<request_data>")]
async fn post_assignment_solution(
    assignment_id: String,
    request_data: Form<PostAssignmetSolutionRequestData<'_>>,
    conn: crate::db::DbConn,
    session: Session
) -> Result<PostAssignmetSolutionResponse, PostAssignmetSolutionError> {
    let request_data = request_data.into_inner();
    let mut sln = request_data.solution.0;
    let user_ids: Vec<String> = request_data.coauthors_user_ids.into_iter()
        .chain(std::iter::once(session.user_id))
        .unique()
        .collect();
    let role_id = request_data.role_id;
    let solution_file = request_data.solution_file;

    let mut solution_buffer = Vec::new();
    let _ = solution_file.open().await?.read_to_end(&mut solution_buffer).await?;

    conn.run(move |c| -> Result<_, PostAssignmetSolutionError> {
        let user_and_coauthors_belong_to_role: bool = {
            let cnt: i64 = user_role::table
                .inner_join(subject_role::table.on(subject_role::role_id.eq(user_role::role_id)))
                .inner_join(assignments::table.on(assignments::subject_id.eq(subject_role::subject_id)))
                .filter(assignments::assignment_id.eq(&assignment_id))
                .filter(user_role::role_id.eq(role_id))
                .filter(user_role::user_id.eq_any(&user_ids))
                .count()
                .get_result(c)?;

            cnt == user_ids.len() as i64
        };

        if !user_and_coauthors_belong_to_role {
            return Err(PostAssignmetSolutionError::BadRequest(()))
        }

        
        let solution_id = Uuid::new_v4().to_string();
        sln.solution_id = solution_id.clone();
        sln.submission_date = Utc::now().naive_utc();
        sln.assignment_id = assignment_id;
        sln.solution_data = solution_buffer;

        let _result = diesel::insert_into(solutions::table)
            .values(sln)
            .execute(c)?;

        let user_solutions: Vec<_> = user_ids.into_iter()
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