use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::Deserialize;

use crate::model::state;
use crate::model::user::User;
use crate::utils::jwt::mint_rsa;
use crate::utils::password::{hash_password, verify_password};

#[derive(Deserialize)]
pub struct ClientRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/register", data = "<data>", format = "json")]
pub async fn register(
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Custom<&'static str> {
    let user = match User::fetch(data.email, state).await {
        Ok(Some(_)) => return Custom(Status::Conflict, "User already exists."),
        Ok(None) => User::new(data.email, &hash_password(data.password)),
        Err(e) => panic!("If you're seeing this message, you fucked up. Reading a user from the database failed: {:?}", e),
    };

    match user.save(state).await {
        true => Custom(Status::Created, "Created"),
        false => Custom(Status::InternalServerError, "Internal Server Error"),
    }
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login(state: &State<state::State>, data: Json<ClientRequest<'_>>) -> Custom<String> {
    let user = User::fetch(data.email, state).await;

    match user {
        Ok(Some(mut user)) => {
            if verify_password(data.password, user.hashed_password.as_str()) {
                let jwt = mint_rsa(&state.rsa_key, &user.email, &user.email);

                match user.login().save(state).await {
                    true => Custom(Status::Ok, jwt),
                    false => Custom(
                        Status::InternalServerError,
                        String::from("Internal server error"),
                    ),
                }
            } else {
                Custom(
                    Status::BadRequest,
                    String::from(
                        "The provided email address and password combination is incorrect.",
                    ),
                )
            }
        },
        Ok(None) => Custom(
            Status::BadRequest,
            String::from("The provided email address and password combination is incorrect."),
        ),
        Err(e) => panic!("If you're seeing this message, you fucked up. Reading a user from the database failed. Error: {:?}", e),
    }
}
