use crate::guards::api_key::ApiKey;
use crate::model::rule::Rule;

use rocket::{get, post};
use rocket::serde::json::{Json, Value};
use rocket::serde::uuid::Uuid;
use rocket::response::status::Created;
use rocket::http::Status;
use serde_json::Map;

#[get("/rule/<format>/<uuid>")]
pub async fn get_rule(api_key: ApiKey<'_>, format: &str, uuid: Uuid) -> Result<Json<Map<String, Value>>, Status> {
    let result = Rule::get(format, uuid, api_key.value).await;

    match result {
        Ok(rule) => {
            let mut response = Map::new();
            response.insert(String::from("uuid"), Value::String(rule.sk.clone()));
            response.insert(String::from("expr"), Value::String(rule.expr.clone()));
            response.insert(String::from("id"), Value::String(rule.id.clone()));

            Ok(Json(response))
        },
        Err(error) => match error.as_str() {
            "Not Found" => Err(Status::NotFound),
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        },
    }
}

#[post("/rule/<format>", data = "<data>")]
pub async fn put_rule(api_key: ApiKey<'_>, format: &str, data: Json<Map<String, Value>>) -> Result<Created<Json<Map<String, Value>>>, Status> {
    let id = data.get("id").unwrap().as_str().unwrap();
    let expr = data.get("expr").unwrap().as_str().unwrap();

    let rule = Rule::new(
        format,
        api_key.value,
        id,
        expr,
    );
    let result = Rule::put(&rule).await;

    match result {
        Ok(uuid) => {
            let mut response = Map::new();
            response.insert(String::from("id"), Value::String(id.to_string()));
            response.insert(String::from("uuid"), Value::String(uuid.to_string()));
            response.insert(String::from("expr"), Value::String(expr.to_string()));

            Ok(Created::new(format!("/rule/{}", format)).body(Json(response)))
        },
        Err(error) => match error.as_str() {
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        },
    }
}
