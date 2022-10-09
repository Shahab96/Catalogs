use aws_sdk_dynamodb::Error::ConditionalCheckFailedException;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::Deserialize;

// use crate::guards::user::AuthenticatedUser;
// use crate::model::role::Role;
use crate::model::state;
use crate::model::user::User;
use crate::utils::jwt::mint_rsa;
use crate::utils::password::verify_password;

#[derive(Deserialize)]
pub struct ClientRequest<'a> {
    email: &'a str,
    password: &'a str,
}

// #[derive(Deserialize)]
// pub struct AddRolesRequest {
//     roles: Vec<String>,
// }

#[post("/register", data = "<data>", format = "json")]
pub async fn register(
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Custom<&'static str> {
    let user = User::new(data.email, data.password);

    match user.save(state).await {
        Ok(_) => Custom(Status::Created, "Created"),
        Err(err) => match err {
            ConditionalCheckFailedException(_) => Custom(Status::Conflict, "User already exists."),
            _ => Custom(Status::InternalServerError, "Internal Server Error"),
        },
    }
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login(state: &State<state::State>, data: Json<ClientRequest<'_>>) -> Custom<String> {
    let user = User::fetch(data.email, state).await;

    match user {
        Ok(Some(mut user)) => {
            if verify_password(data.password, user.hashed_password.as_str()) {
                let jwt = mint_rsa(&state.rsa_key, &user.email, &user.email);
                if !jwt.is_ok() {
                    return Custom(
                        Status::InternalServerError,
                        String::from("If you're seeing this message, you fucked up. The JWT minter is broken."),
                    );
                }

                user.login(state).await.unwrap();
                Custom(Status::Ok, jwt.unwrap())
            } else {
                Custom(
                    Status::BadRequest,
                    String::from(
                        "The provided email address and password combination is incorrect.",
                    ),
                )
            }
        }
        Ok(None) => Custom(
            Status::BadRequest,
            String::from("The provided email address and password combination is incorrect."),
        ),
        Err(_) => Custom(
            Status::InternalServerError,
            String::from("Internal Server Error"),
        ),
    }

    // if let Ok(mut user) = user {
    //     if verify_password(data.password, user.hashed_password.as_str()) {
    //         let jwt = mint_rsa(&state.rsa_key, &data.email, &user.email);
    //         if !jwt.is_ok() {
    //             return Custom(
    //                 Status::InternalServerError,
    //                 String::from(
    //                     "If you're seeing this message, you fucked up. The JWT minter is broken.",
    //                 ),
    //             );
    //         }

    //         Custom(Status::Ok, jwt.unwrap())
    //     }
    // } else {
    //     println!("If you're seeing this message, you fucked up. A user login failed somehow.");
    //     Custom(
    //         Status::InternalServerError,
    //         String::from("Internal Server Error"),
    //     )
    // }
    //     } else {
    //         Custom(
    //             Status::Unauthorized,
    //             String::from("Incorrect email or password."),
    //         )
    //     }
    // } else {
    //     Custom(
    //         Status::InternalServerError,
    //         String::from("Internal Server Error"),
    //     )
    // }
}
