mod model;
mod routes;
mod utils;

use crate::model::state::State;
use crate::routes::oauth::{oauth_authorization, oauth_login};
use crate::routes::user::{login, register};

use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use rocket::{self, routes};

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    let rsa_key_secret = std::env::var("RSA_KEY_SECRET").unwrap();
    let google_oauth_credentials_secret = std::env::var("GOOGLE_OAUTH_CREDENTIALS").unwrap();
    let table_name = std::env::var("TABLE_NAME").unwrap();
    let oauth_redirect_uri = std::env::var("OAUTH_REDIRECT_URI").unwrap();

    let config = aws_config::load_from_env().await;
    let dynamo = aws_sdk_dynamodb::Client::new(&config);
    let secrets_manager = aws_sdk_secretsmanager::Client::new(&config);

    let rsa_key = secrets_manager
        .get_secret_value()
        .secret_id(rsa_key_secret)
        .send()
        .await
        .unwrap()
        .secret_string
        .unwrap();

    let google_oauth_credentials = serde_json::from_str(
        secrets_manager
            .get_secret_value()
            .secret_id(google_oauth_credentials_secret)
            .send()
            .await
            .unwrap()
            .secret_string()
            .unwrap(),
    )?;

    let state = State {
        dynamo,
        table_name,
        rsa_key,
        google_oauth_credentials,
        oauth_redirect_uri,
    };

    let rocket = rocket::build()
        .configure(rocket::Config::debug_default())
        .manage(state)
        .mount(
            "/",
            routes![register, login, oauth_login, oauth_authorization],
        );

    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        let _ = rocket.launch().await?;
    }

    Ok(())
}
