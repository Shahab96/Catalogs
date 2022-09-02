mod guards;
mod model;
mod routes;

use crate::routes::rules::{create_rule, get_rule, list_rules};
use aws_sdk_dynamodb::Client;
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
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
                .mount("/", routes![get_rule, create_rule, list_rules]);
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
