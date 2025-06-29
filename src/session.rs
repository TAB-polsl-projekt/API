use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl};
use okapi::{openapi3::{RefOr, Response, Responses}, Map};
use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest, Request}};
use rocket_okapi::{r#gen::OpenApiGenerator, request::{OpenApiFromRequest, RequestHeaderInput}};

use crate::{db::DbConn, dbmodels::SessionId, schema::{session_ids, users}};

pub struct Session {
    pub user_id: String,
    pub session_id: String,
    pub is_admin: bool
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

        let srk_result = db.run(move |c| {
            session_ids::table
                .filter(session_ids::refresh_key_id.eq(&session_id))
                .select(session_ids::all_columns)
                .first(c)
        }).await;

        let srk: SessionId = match srk_result {
            Ok(srk) => srk,
            Err(err) => match err {
                diesel::result::Error::NotFound => return Outcome::Error((Status::Unauthorized, ())),
                _ => return Outcome::Error((Status::InternalServerError, ())),
            }
        };

        let session_id = srk.refresh_key_id;
        let user_id = srk.user_id.clone();

        let is_admin = db.run(move |c| {
            users::table
                .filter(users::user_id.eq(srk.user_id))
                .select(users::is_admin)
                .first(c)
        }).await
        .unwrap_or(false);

        Outcome::Success(
            Session {
                session_id,
                user_id,
                is_admin
            }
        )
    }
}

impl<'r> OpenApiFromRequest<'r> for Session {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }

    fn get_responses(
        _gen: &mut OpenApiGenerator,
    ) -> rocket_okapi::Result<Responses> {
        // Define possible responses: 401 Unauthorized, 500 Internal Server Error
        let mut responses = Responses::default();

        responses.responses.insert(
            "401".to_owned(),
            RefOr::Object(Response {
                description: "Unauthorized: missing or invalid session cookie".to_owned(),
                headers: Map::new(),
                content: Map::new(),
                links: Map::new(),
                extensions: Map::new(),
            }),
        );

        responses.responses.insert(
            "500".to_owned(),
            RefOr::Object(Response {
                description: "Internal server error".to_owned(),
                headers: Map::new(),
                content: Map::new(),
                links: Map::new(),
                extensions: Map::new(),
            }),
        );

        Ok(responses)
    }
}