mod api;
mod model;
mod repository;

use api::rule::get_rule;
use repository::dynamo::Dynamo;

use actix_web::{self, middleware::Logger, web::Data, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let config = aws_config::load_from_env().await;
    let key = "TABLE_NAME";

    let factory = move || {
        let dynamo_client: Dynamo;
        match std::env::var(key) {
            Ok(table_name) => dynamo_client = Dynamo::init(table_name, config.clone()),
            Err(_) => panic!("{} variable not set", key),
        }
        let data = Data::new(dynamo_client);
        let logger = Logger::default();
        App::new().wrap(logger).app_data(data).service(get_rule)
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        println!("Running locally");
        HttpServer::new(factory)
            .bind("127.0.0.1:8000")?
            .run()
            .await?;
    }
    Ok(())
}
