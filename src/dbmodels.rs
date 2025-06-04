use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, AsChangeset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dbschema::users;

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: Option<String>,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub student_id: Option<String>,
    pub user_disabled: Option<bool>,
    pub last_login_time: Option<NaiveDateTime>
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub student_id: Option<String>,
    pub user_disabled: Option<bool>,
    pub last_login_time: Option<NaiveDateTime>
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = assignments)]
pub struct Assignment {
    pub assignment_id: Option<String>,
    pub subject_id: String,
    pub title: Option<String>,
    pub description: Option<String>
}