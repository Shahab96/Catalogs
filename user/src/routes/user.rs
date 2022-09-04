use aws_sdk_dynamodb::Error::ConditionalCheckFailedException;
use rocket::http::Status;
use rocket::response::status::{Accepted, Created, Custom};
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::Deserialize;

use crate::model::user::User;
use crate::model::tenant::Tenant;
use crate::model::state;
use crate::utils::utils::{verify_password, mint_token};

#[derive(Deserialize)]
pub struct ClientRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/register", data = "<data>", format = "json")]
pub async fn register(
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Result<Created<()>, Custom<&'static str>> {
    let user = User::new(data.email, data.password);
    let result = User::create(&user, state).await;

    match result {
        Ok(_) => {
            let tenant = Tenant::new(&user.email, &user.tenant_list.first().unwrap());
            match Tenant::create(&tenant, state).await {
                Ok(_) => Ok(Created::new("/")),
                Err(_) => Err(Custom(Status::InternalServerError, "Error creating your account.")),
            }
        },
        Err(err) => match err {
            ConditionalCheckFailedException(_) => {
                Err(Custom(Status::Conflict, "User already exists."))
            }
            _ => Err(Custom(Status::InternalServerError, "Internal Server Error")),
        },
    }
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login<'a> (
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Result<Accepted<String>, Custom<&'a str>> {
    let result = User::login(data.email, state).await;

    match result {
        Ok(user) => {
            if verify_password(data.password) {
                let tenant_id = user.attributes().unwrap().get("active_tenant").unwrap().as_s().unwrap();
                let jwt = mint_token(&state.access_token, &data.email, &tenant_id).unwrap();
                
                Ok(Accepted(Some(String::from(jwt.as_str()))))
            } else {
                Err(Custom(Status::Unauthorized, "Incorrect email or password."))
            }
        },
        Err(err) => match err {
            ConditionalCheckFailedException(_) => Err(Custom(Status::NotFound, "Incorrect email or password.")),
            _ => Err(Custom(Status::InternalServerError, "Internal Server Error")),
        },
    }
}
