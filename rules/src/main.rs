mod guards;
mod model;
mod routes;

use crate::routes::rules::get_rule;
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use rocket::{self, routes};

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    let rocket = rocket::build().mount("/", routes![get_rule]);
    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        let _ = rocket.launch().await?;
    }
    Ok(())
}
