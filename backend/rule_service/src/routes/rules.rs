use std::str::FromStr;

use crate::guards::api_key::ApiKey;
use crate::model::rule::Rule;

use aws_sdk_dynamodb::Client;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::{Json, Value};
use rocket::serde::uuid::Uuid;
use rocket::{get, post, State};
use serde_json::Map;

#[get("/rule/<format>/<uuid>")]
pub async fn get_rule(
    api_key: ApiKey<'_>,
    format: &str,
    uuid: &str,
    client: &State<Client>,
    table_name: &State<String>,
) -> Result<Json<Map<String, Value>>, Status> {
    let result = Rule::get(
        format,
        Uuid::from_str(uuid).unwrap(),
        api_key.value,
        client,
        &table_name,
    )
    .await;

    match result {
        Ok(rule) => {
            let mut response = Map::new();
            response.insert(String::from("uuid"), Value::String(rule.sk.clone()));
            response.insert(String::from("expr"), Value::String(rule.expr.clone()));
            response.insert(String::from("id"), Value::String(rule.id.clone()));

            Ok(Json(response))
        }
        Err(error) => match error.as_str() {
            "Not Found" => Err(Status::NotFound),
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        },
    }
}

#[post("/rule/<format>", data = "<data>", format = "json")]
pub async fn create_rule(
    api_key: ApiKey<'_>,
    format: &str,
    data: Json<Map<String, Value>>,
    client: &State<Client>,
    table_name: &State<String>,
) -> Result<Created<Json<Map<String, Value>>>, Status> {
    let id = data.get("id").unwrap().as_str().unwrap();
    let expr = data.get("expr").unwrap().as_str().unwrap();

    let rule = Rule::new(format, api_key.value, id, expr);
    let result = Rule::create(&rule, &client, &table_name).await;

    match result {
        Ok(uuid) => {
            let mut response = Map::new();
            response.insert(String::from("id"), Value::String(id.to_string()));
            response.insert(String::from("uuid"), Value::String(uuid.to_string()));
            response.insert(String::from("expr"), Value::String(expr.to_string()));

            Ok(Created::new(format!("/rule/{}", format)).body(Json(response)))
        }
        Err(error) => match error.as_str() {
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        }
    }
}

#[get("/rules/<format>")]
pub async fn list_rules(
    api_key: ApiKey<'_>,
    format: &str,
    client: &State<Client>,
    table_name: &State<String>,
) -> Result<Json<Vec<Map<String, Value>>>, Status> {
    let result = Rule::list(format, api_key.value, client, &table_name).await;

    match result {
        Ok(rules) => {
            let mut response = Vec::new();
            for rule in rules {
                let mut rule_response = Map::new();
                rule_response.insert(String::from("uuid"), Value::String(rule.sk.clone()));
                rule_response.insert(String::from("expr"), Value::String(rule.expr.clone()));
                rule_response.insert(String::from("id"), Value::String(rule.id.clone()));

                response.push(rule_response);
            }

            Ok(Json(response))
        }
        Err(error) => match error.as_str() {
            "Not Found" => Err(Status::NotFound),
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        }
    }
}
