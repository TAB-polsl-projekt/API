use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, AsChangeset};
use schemars::JsonSchema;
use serde::{Serialize};

use crate::dbschema::{assigments, solution, subjects, users};

use serde::{Deserialize, Deserializer};

pub fn from_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))?)
}

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

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, Insertable)]
#[diesel(table_name = assigments)]
pub struct Assignment {
    pub assigment_id: Option<String>,
    pub subject_id: String,
    pub title: Option<String>,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, AsChangeset)]
#[diesel(table_name = assigments)]
pub struct AssignmentUpdate {
    pub title: Option<String>,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = subjects)]
pub struct Subject {
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub editor_role_id: String
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, AsChangeset)]
#[diesel(table_name = subjects)]
pub struct SubjectUpdate {
    pub subject_name: Option<String>,
    pub editor_role_id: Option<String>
}


#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = solution)]
pub struct Solution {
    pub solution_id: Option<String>,
    #[serde(skip)]
    pub grade: Option<f64>,
    #[serde(deserialize_with = "from_timestamp")]
    #[serde(skip)]
    pub submission_date: Option<NaiveDateTime>,
    #[serde(serialize_with = "serde_bytes::serialize")]
    #[serde(deserialize_with = "serde_bytes::deserialize")]
    pub solution_data: Option<Vec<u8>>,
    #[serde(skip)]
    pub reviewed_by: Option<String>,
    #[serde(deserialize_with = "from_timestamp")]
    #[serde(skip)]
    pub review_date: Option<NaiveDateTime>
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = session_refresh_keys)]
pub struct SessionRefreshKeys {
    pub refresh_key_id: Option<String>,
    pub user_id: String,
    pub expiration_time: Option<NaiveDateTime>,
    pub refresh_count: Option<i32>,
    pub refresh_limit: Option<i32>,
}