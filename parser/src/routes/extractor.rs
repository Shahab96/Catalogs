use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde_json::{Map, Value};

use crate::model::event::EvaluationEvent;
use crate::service::parser::Parser;

#[post("/evaluate", format = "json", data = "<message>")]
pub async fn evaluate(message: Json<EvaluationEvent>) -> Result<Json<Map<String, Value>>, Status> {
    println!("{:?}", message.0);
    let mut parser = Parser::new(message.0.rule);

    match parser.parse(message.0.sample) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}
