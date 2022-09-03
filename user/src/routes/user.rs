use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::Error::ConditionalCheckFailedException;
use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::{Deserialize, Serialize};

use crate::model::user::User;

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/register", data = "<data>", format = "json")]
pub async fn register(
    client: &State<Client>,
    table_name: &State<String>,
    data: Json<RegisterRequest<'_>>,
) -> Result<Created<()>, Custom<&'static str>> {
    let tenant = User::new(data.email, data.password);
    let result = User::create(&tenant, client, &table_name).await;

    match result {
        Ok(_) => Ok(Created::new("/register")),
        Err(err) => match err {
            ConditionalCheckFailedException(_) => Err(Custom(Status::Conflict, "User already exists.")),
            _ => Err(Custom(Status::InternalServerError, "Internal Server Error")),
        }
    }
}
