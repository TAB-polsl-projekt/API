use okapi::{openapi3::{RefOr, Response, Responses}, Map};
use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest, Request}};
use rocket_okapi::{r#gen::OpenApiGenerator, request::{OpenApiFromRequest, RequestHeaderInput}};

use crate::session::Session;

#[allow(dead_code)]
pub struct AdminSession {
    session: Session
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminSession {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_outcome = Session::from_request(request).await;
        let session = match session_outcome {
            Outcome::Success(session) => session,
            Outcome::Error(error) => return Outcome::Error(error),
            Outcome::Forward(forward) => return Outcome::Forward(forward),
        };

        if !session.is_admin {
            return Outcome::Error((Status::Forbidden, ()));
        }

        let admin_session = AdminSession {session};

        Outcome::Success(admin_session)
    }
}

impl<'r> OpenApiFromRequest<'r> for AdminSession {
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
        let mut responses = Session::get_responses(_gen)?;

        responses.responses.insert(
            "403".to_owned(),
            RefOr::Object(Response {
                description: "Forbidden: currently logged-in user is not an admin".to_owned(),
                headers: Map::new(),
                content: Map::new(),
                links: Map::new(),
                extensions: Map::new(),
            }),
        );

        Ok(responses)
    }
}