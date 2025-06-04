use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{Utc, Duration};
use reqwest::Client;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::db::DbConn;

use crate::dbschema::{users, microsoft_logins, session_refresh_keys};

#[derive(Deserialize)]
pub struct MicrosoftTokenRequest {
    access_token: String,
}

#[derive(Deserialize, Debug)]
struct MicrosoftGraphUser {
    id: String,
    displayName: String,
    mail: Option<String>,
    userPrincipalName: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize)]
struct JwtClaims {
    sub: String,
    name: String,
    email: String,
    exp: usize,
}

#[post("/auth/microsoft")]
pub async fn microsoft_auth(
    conn: web::Data<DbConn>,
    payload: web::Json<MicrosoftTokenRequest>,
) -> HttpResponse {
    let client = Client::new();
    let user_info_resp = client
        .get("https://graph.microsoft.com/v1.0/me")
        .bearer_auth(&payload.access_token)
        .send()
        .await;

    if let Ok(resp) = user_info_resp {
        if resp.status().is_success() {
            let ms_user: MicrosoftGraphUser = resp.json().await.unwrap();
            let email = ms_user.mail.clone().unwrap_or(ms_user.userPrincipalName.clone());

            let result = conn.run(move |c| {
                c.transaction::<_, diesel::result::Error, _>(|| {
                    use users::dsl::*;
                    use microsoft_logins::dsl::*;

                    let existing_user_id = microsoft_logins
                        .filter(microsoft_id.eq(&ms_user.id))
                        .select(user_id)
                        .first::<Option<String>>(c)
                        .optional()?
                        .flatten();

                    let uid = match existing_user_id {
                        Some(uid) => uid,
                        None => {
                            let new_user_id = Uuid::new_v4().to_string();
                            diesel::insert_into(users)
                                .values((
                                    user_id.eq(&new_user_id),
                                    email.eq(&email),
                                    name.eq(ms_user.displayName.clone()),
                                    surname.eq(""), 
                                ))
                                .execute(c)?;

                            diesel::insert_into(microsoft_logins)
                                .values((
                                    microsoft_id.eq(&ms_user.id),
                                    user_id.eq(&new_user_id),
                                ))
                                .execute(c)?;

                            new_user_id
                        }
                    };

                    let refresh_token = Uuid::new_v4().to_string();
                    diesel::insert_into(session_refresh_keys::table)
                        .values((
                            session_refresh_keys::refresh_key_id.eq(&refresh_token),
                            session_refresh_keys::user_id.eq(&uid),
                            session_refresh_keys::expiration_time.eq(Some(Utc::now().naive_utc() + Duration::days(7))),
                        ))
                        .execute(c)?;

                    Ok((uid, refresh_token))
                })
            }).await;

            return match result {
                Ok((uid, refresh_token)) => {
                    let jwt_claims = JwtClaims {
                        sub: uid.clone(),
                        name: ms_user.displayName.clone(),
                        email: email.clone(),
                        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
                    };

                    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                    let jwt_token = encode(&Header::default(), &jwt_claims, &EncodingKey::from_secret(jwt_secret.as_bytes())).unwrap();

                    HttpResponse::Ok().json(TokenResponse {
                        access_token: jwt_token,
                        refresh_token,
                    })
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to create session"),
            };
        }
    }

    HttpResponse::Unauthorized().body("Invalid Microsoft token")
}
