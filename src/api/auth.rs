use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::dbschema::session_refresh_keys::dsl::*;
use crate::dbschema::users::dsl::*;

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(Serialize)]
pub struct JwtResponse {
    access_token: String,
}

#[derive(Serialize)]
struct JwtClaims {
    sub: String,
    name: String,
    email: String,
    exp: usize,
}

#[post("/auth")]
pub async fn validate_refresh_token(
    conn: web::Data<DbConn>,
    payload: web::Json<RefreshTokenRequest>,
) -> HttpResponse {
    let token = payload.refresh_token.clone();

    let result = conn.run(move |c| {

        let entry = session_refresh_keys
            .filter(refresh_key_id.eq(&token))
            .first::<(Option<String>, String, Option<chrono::NaiveDateTime>, Option<i32>, Option<i32>)>(c)
            .optional()?;

        let (_, uid, _, _, _) = match entry {
            Some(r) => r,
            None => return Err(diesel::result::Error::NotFound),
        };

        diesel::delete(session_refresh_keys.filter(refresh_key_id.eq(&token)))
            .execute(c)?;

        let user = users
            .filter(user_id.eq(&uid))
            .first::<(Option<String>, String, String, String, Option<String>, Option<bool>, Option<chrono::NaiveDateTime>)>(c)?;

        let (_, _, name_val, surname_val, _, _, _) = user;

        Ok((uid, name_val, surname_val))
    }).await;

    match result {
        Ok((uid, name, surname)) => {
            let claims = JwtClaims {
                sub: uid,
                name: format!("{} {}", name, surname),
                email: "".into(), 
                exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            };

            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
                .expect("Failed to encode JWT");

            HttpResponse::Ok().json(JwtResponse {
                access_token: token,
            })
        },
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::Unauthorized().body("Invalid or expired refresh token")
        },
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
