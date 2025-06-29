use diesel::dsl::exists;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{delete, get, post};
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::{Role, User};
use crate::schema::{roles, user_role, users};
use crate::{define_api_response, define_response_data};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        post_role,
        delete_role,
        get_role_users,
        get_roles
    ]
}
define_api_response!(pub enum PostRoleResponse {
    Ok => (200, "Role has been added", (), ()),
});

define_api_response!(pub enum PostRoleError {
    InternalServerError => (500, "Unexpected server error", String, (diesel::result::Error)),
});

#[openapi(tag = "Roles")]
#[post("/roles", data = "<role>")]
pub async fn post_role(role: Json<Role>, conn: crate::db::DbConn, _session: AdminSession) -> Result<PostRoleResponse, PostRoleError> {
    let role = role.0;
    
    let _ = conn.run(move |c|{
        diesel::insert_into(roles::table)
            .values(role)
            .execute(c)
    }).await?;

    Ok(PostRoleResponse::Ok(()))
}

define_api_response!(pub enum DeleteRoleResponse {
    Ok => (200, "Role has been deleted", (), ()),
});

define_api_response!(pub enum DeleteRoleError {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

/// Delete a role
#[openapi(tag = "Roles", operation_id = "deleteRole")]
#[delete("/roles/<role_id>")]
pub async fn delete_role(role_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<DeleteRoleResponse, DeleteRoleError> {
    let _result = conn.run(move |c| {
        diesel::delete(roles::table)
            .filter(roles::role_id.eq(role_id))
            .execute(c)
    }).await?;

    Ok(DeleteRoleResponse::Ok(()))
}

define_api_response!(pub enum GetRoleUsersResponse {
    Ok => (200, "Return users which are members of the role", Vec<GetRoleUsersResponseData>, ()),
});

define_api_response!(pub enum GetRoleUsersError {
    Unauthorized => (401, "This user cannot view users in other roles", (), ()),
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(
    pub struct GetRoleUsersResponseData {
        pub user_id: String,
        pub full_name: String,
        pub student_id: String,
    }
);


#[openapi(tag = "Roles")]
#[get("/roles/<role_id>/users")]
pub async fn get_role_users(role_id: String, conn: crate::db::DbConn, session: Session) -> Result<GetRoleUsersResponse, GetRoleUsersError> {
    let result: Vec<User> = conn.run(move |c| {
        if !session.is_admin {
            let can_view_role: bool = diesel::select(exists(
                user_role::table
                    .filter(user_role::user_id.eq(&session.user_id))
                    .filter(user_role::role_id.eq(&role_id))
            )).get_result(c)?;

            if !can_view_role {
                return Err(GetRoleUsersError::Unauthorized(()));
            }
        }

        let users = user_role::table
            .inner_join(users::table.on(users::user_id.eq(user_role::user_id)))
            .filter(user_role::role_id.eq(&role_id))
            .filter(user_role::user_id.ne(&session.user_id))
            .select(users::all_columns)
            .load(c)?;

        Ok(users)
    }).await?;

    let response_data = result.into_iter()
        .map(|x| {
            let user_id = x.user_id;
            let full_name = format!("{} {}", x.name, x.surname);
            let student_id = x.student_id.unwrap_or("".to_string());

            GetRoleUsersResponseData { user_id, full_name, student_id }
        })
        .collect();

    Ok(GetRoleUsersResponse::Ok(response_data))
}

define_api_response!(pub enum GetRolesResponse {
    Ok => (200, "Return all roles", Vec<Role>, ()),
});

define_api_response!(pub enum GetRolesError {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

#[openapi(tag = "Roles")]
#[get("/roles")]
pub async fn get_roles(conn: crate::db::DbConn, _session: AdminSession) -> Result<GetRolesResponse, GetRolesError> {
    let roles = conn.run(move |c|{
        roles::table.load(c)
    }).await?;

    Ok(GetRolesResponse::Ok(roles))
}