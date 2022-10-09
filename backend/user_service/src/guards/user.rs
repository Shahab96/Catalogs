use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::utils::jwt::get_claims;

pub struct AuthenticatedUser {
    pub email: String,
    pub tenant_id: String,
}

#[derive(Debug)]
pub enum AuthenticationError {
    Missing,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authentication") {
            None => Outcome::Failure((Status::BadRequest, AuthenticationError::Missing)),
            Some(token) => {
                let (email, tenant_id) = get_claims(token).unwrap();
                Outcome::Success(AuthenticatedUser { email, tenant_id })
            }
        }
    }
}
