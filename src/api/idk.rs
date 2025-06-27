use diesel::dsl::{exists, not};
use diesel::{delete, insert_into, update, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use rocket::{get, post, delete, put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use uuid::Uuid;

use crate::dbmodels::{Assignment, Solution, User};
use crate::schema::{
    assignments,
    roles,
    subjects,
    user_role,
    subject_role,
    users,
    solutions,
    user_solution,
};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        get_not_enrolled,
        enroll_user,
        unenroll_user,
        create_assignment,
        delete_assignment,
        get_user_solution,
        update_user_solution,
        get_subject_assignment
    ]
}

define_api_response!(pub enum Response {
    Ok => (200, "Success", (), ()),
    Created => (201, "Created", (), ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "Unauthorized", (), ()),
    NotFound => (404, "Not Found", (), ()),
    InternalServerError => (500, "Internal error", String, (diesel::result::Error)),
});

// 6. GET /subjects/<subject_id>/users/not-enrolled
#[openapi(tag = "Subject")]
#[get("/subjects/<subject_id>/users/not-enrolled")]
pub async fn get_not_enrolled(
    subject_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Json<Vec<User>>, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    let users_list = conn
        .run(move |c| {
            users::table
                .filter(not(exists(
                    user_role::table
                        .inner_join(subject_role::table.on(
                            subject_role::role_id.eq(user_role::role_id),
                        ))
                        .filter(user_role::user_id.eq(users::user_id))
                        .filter(subject_role::subject_id.eq(&subject_id)),
                )))
                .select(users::all_columns)
                .get_results::<User>(c)
        })
        .await?;
    Ok(Json(users_list))
}

// 7. POST /subjects/<subject_id>/users/<user_id>
#[openapi(tag = "Subject")]
#[post("/subjects/<subject_id>/users/<user_id>")]
pub async fn enroll_user(
    subject_id: String,
    user_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    // Assign 'student' role to user
    conn.run(move |c| {
        insert_into(user_role::table)
            .values((
                user_role::user_id.eq(user_id.clone()),
                user_role::role_id.eq("student"),
            ))
            .execute(c)
    })
    .await?;
    Ok(Response::Created(()))
}

// 8. DELETE /subjects/<subject_id>/users/<user_id>
#[openapi(tag = "Subject")]
#[delete("/subjects/<subject_id>/users/<user_id>")]
pub async fn unenroll_user(
    subject_id: String,
    user_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    // Remove any user_role entries for roles associated with this subject
    let count = conn
        .run(move |c| {
            delete(
                user_role::table
                    .filter(user_role::user_id.eq(user_id.clone()))
                    .filter(exists(
                        subject_role::table
                            .filter(subject_role::role_id.eq(user_role::role_id))
                            .filter(subject_role::subject_id.eq(&subject_id)),
                    )),
            )
            .execute(c)
        })
        .await?;
    if count == 0 {
        return Err(Error::NotFound(()));
    }
    Ok(Response::Ok(()))
}

// 9. POST /subjects/<subject_id>/assignments
#[derive(serde::Deserialize, JsonSchema)]
pub struct CreateAssignmentRequest {
    pub title: String,
    pub description: String,
    pub accepted_mime_types: Option<String>,
}

#[openapi(tag = "Assignment")]
#[post("/subjects/<subject_id>/assignments", format = "json", data = "<body>")]
pub async fn create_assignment(
    subject_id: String,
    body: Json<CreateAssignmentRequest>,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    let new_id = Uuid::new_v4().to_string();
    let mimes = body
        .accepted_mime_types
        .clone()
        .unwrap_or_else(|| "[]".to_string());
    conn.run(move |c| {
        insert_into(assignments::table)
            .values((
                assignments::assignment_id.eq(new_id.clone()),
                assignments::subject_id.eq(subject_id.clone()),
                assignments::title.eq(&body.title),
                assignments::description.eq(&body.description),
                assignments::accepted_mime_types.eq(mimes),
            ))
            .execute(c)
    })
    .await?;
    Ok(Response::Created(()))
}

// 10. DELETE /subjects/<subject_id>/assignments/<assignment_id>
#[openapi(tag = "Assignment")]
#[delete("/subjects/<subject_id>/assignments/<assignment_id>")]
pub async fn delete_assignment(
    subject_id: String,
    assignment_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    let count = conn
        .run(move |c| {
            delete(
                assignments::table.filter(
                    assignments::assignment_id
                        .eq(assignment_id.clone())
                        .and(assignments::subject_id.eq(subject_id.clone())),
                ),
            )
            .execute(c)
        })
        .await?;
    if count == 0 {
        return Err(Error::NotFound(()));
    }
    Ok(Response::Ok(()))
}

// 11. GET /users/<user_id>/assignments/<assignment_id>/solution
#[openapi(tag = "Solution")]
#[get("/users/<user_id>/assignments/<assignment_id>/solution")]
pub async fn get_user_solution(
    user_id: String,
    assignment_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Json<Option<Solution>>, Error> {
    if !session.is_admin && session.user_id != user_id {
        return Err(Error::Unauthorized(()));
    }
    let sol = conn
        .run(move |c| {
            user_solution::table
                .inner_join(solutions::table.on(solutions::solution_id.eq(
                    user_solution::solution_id,
                )))
                .select(solutions::all_columns)
                .filter(user_solution::user_id.eq(user_id.clone()))
                .filter(solutions::assignment_id.eq(assignment_id.clone()))
                .first(c)
                .optional()
        })
        .await?;
    Ok(Json(sol))
}

// 12. PUT /users/<user_id>/assignments/<assignment_id>/solution
#[derive(serde::Deserialize, JsonSchema)]
pub struct UpdateSolutionRequest {
    pub grade: f64,
    pub review_comment: String,
}

#[openapi(tag = "Solution")]
#[put("/users/<user_id>/assignments/<assignment_id>/solution", format = "json", data = "<body>")]
pub async fn update_user_solution(
    user_id: String,
    assignment_id: String,
    body: Json<UpdateSolutionRequest>,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Response, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    let count = conn
        .run(move |c| {
            let sub = user_solution::table
                .filter(user_solution::user_id.eq(user_id.clone()))
                .select(user_solution::solution_id);
            update(solutions::table.filter(
                solutions::solution_id.eq_any(sub).and(
                    solutions::assignment_id.eq(assignment_id.clone()),
                ),
            ))
            .set((
                solutions::grade.eq(body.grade),
                solutions::review_comment.eq(&body.review_comment),
                solutions::review_date.eq(diesel::dsl::now),
            ))
            .execute(c)
        })
        .await?;
    if count == 0 {
        return Err(Error::NotFound(()));
    }
    Ok(Response::Ok(()))
}

#[openapi(tag = "Subject")]
#[get("/subjects/<subject_id>/assignments")]
pub async fn get_subject_assignment(
    subject_id: String,
    conn: crate::db::DbConn,
    session: Session,
) -> Result<Json<Vec<Assignment>>, Error> {
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }
    let assignment_list = conn
        .run(move |c| {
            subjects::table
                .inner_join(assignments::table.on(
                    assignments::subject_id.eq(subjects::subject_id),
                ))
                .filter(subjects::subject_id.eq(subject_id))
                .select(assignments::all_columns)
                .get_results(c)
        })
        .await?;
    Ok(Json(assignment_list))
}
