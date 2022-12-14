use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde_json::{Map, Value};

use crate::model::event::EvaluationEvent;
use crate::service::extractor::Extractor;

#[post("/evaluate", format = "json", data = "<message>")]
pub async fn evaluate(message: Json<EvaluationEvent>) -> Result<Json<Map<String, Value>>, Status> {
    println!("{:?}", message.0);
    let mut extractor = Extractor::new(message.0.rule);

    match extractor.extract(message.0.sample) {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}
