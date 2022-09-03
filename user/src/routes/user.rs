use aws_sdk_dynamodb::Client;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::{Serialize, Deserialize};

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
) -> Result<Created<()>, Status> {
    let tenant = User::new(data.email, data.password);
    let result = User::create(&tenant, client, &table_name).await;

    match result {
        Ok(_) => Ok(Created::new("/register")),
        Err(_) => Err(Status::InternalServerError),
    }
}
