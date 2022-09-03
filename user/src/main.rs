mod routes;
mod utils;
mod model;

use crate::routes::user::register;

use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use aws_sdk_dynamodb::Client;
use rocket::{self, routes};

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    match std::env::var("TABLE_NAME") {
        Ok(table_name) => {
            let config = aws_config::load_from_env().await;
            let client = Client::new(&config);
            let rocket = rocket::build()
                .manage(client)
                .manage(table_name)
                .mount("/", routes![register]);
            if is_running_on_lambda() {
                // Launch on AWS Lambda
                launch_rocket_on_lambda(rocket).await?;
            } else {
                // Launch local server
                let _ = rocket.launch().await?;
            }
            Ok(())
        }
        Err(_) => Err(LambdaError::from("TABLE_NAME not set")),
    }
}
