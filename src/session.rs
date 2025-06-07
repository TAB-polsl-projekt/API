use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl};
use okapi::openapi3::Responses;
use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest, Request}};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

use crate::{db::DbConn, dbmodels::SessionRefreshKeys, dbschema::session_refresh_keys};

pub struct Session {
    pub user_id: String,
    pub session_id: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = match DbConn::from_request(request).await {
            Outcome::Success(db) => db,
            _ => return Outcome::Error((Status::InternalServerError, ())),
        };

        let session_cookie = request.cookies().get("session_id");
        let session_id = match session_cookie {
            Some(session_id) => session_id.value().to_string(),
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let srk_result: Option<SessionRefreshKeys> = db.run(move |c| {
            session_refresh_keys::table
                .filter(session_refresh_keys::refresh_key_id.eq(&session_id))
                .select(session_refresh_keys::all_columns)
                .first(c)
                .ok()
        }).await;

        let srk = match srk_result {
            Some(srk) => srk,
            None => return Outcome::Error((Status::InternalServerError, ()))
        };

        // TODO: Check if valid
        let session_id = srk.refresh_key_id.unwrap();
        let user_id = srk.user_id;

        Outcome::Success(
            Session {
                session_id,
                user_id,
            }
        )
    }
}

impl<'r> OpenApiFromRequest<'r> for Session {
    fn from_request_input(
            _gen: &mut rocket_okapi::r#gen::OpenApiGenerator,
            _name: String,
            _required: bool,
        ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }

    fn get_responses(_gen: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
        Ok(Responses::default())
    }
}