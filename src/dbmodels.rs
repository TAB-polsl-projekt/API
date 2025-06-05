use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, AsChangeset};
use schemars::JsonSchema;
use serde::{Serialize};

use crate::dbschema::{solution, users};

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

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = assignments)]
pub struct Assignment {
    pub assignment_id: Option<String>,
    pub subject_id: String,
    pub title: Option<String>,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = subject)]
pub struct Subject {
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub editor_role_id: String
}

#[derive(Debug, Queryable, Serialize, Deserialize, JsonSchema, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = solution)]
pub struct Solution {
    pub solution_id: Option<String>,
    pub grade: f64,
    #[serde(deserialize_with = "from_timestamp")]
    pub submission_date: NaiveDateTime,
    #[serde(serialize_with = "serde_bytes::serialize")]
    #[serde(deserialize_with = "serde_bytes::deserialize")]
    pub solution_data: Vec<u8>,
    pub reviewed_by: Option<String>,
    #[serde(deserialize_with = "from_timestamp")]
    pub review_date: NaiveDateTime
}