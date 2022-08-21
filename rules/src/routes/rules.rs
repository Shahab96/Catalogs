use std::collections::HashMap;

use crate::guards::api_key::ApiKey;
use crate::model::rule::Rule;

use rocket::{get, post};
use rocket::serde::json::{Json, Value};
use rocket::serde::uuid::Uuid;
use rocket::response::status::Created;
use rocket::http::Status;

#[get("/rule/<format>/<uuid>")]
pub async fn get_rule<'r>(api_key: ApiKey<'_>, format: &'r str, uuid: Uuid) -> Result<Json<HashMap<&'r str, &'r str>>, Status> {
    let rule = Rule {
        pk: format!("{}#{}", format, api_key.value).as_str(),
        sk: uuid,
        id: None,
        expr: None,
    };
    let result = Rule::get(rule).await;

    match result {
        Ok() => {
            let mut response = HashMap::new();
            response.insert("uuid", rule.sk);
            response.insert("expr", rule.expr);
            response.insert("id", rule.id);

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
pub async fn put_rule<'r>(api_key: ApiKey<'_>, format: &'r str, data: Json<HashMap<String, Value>>) -> Result<Created<Json<HashMap<&'r str, &'r str>>>, Status> {
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
            let mut response = HashMap::new();
            response.insert("id", id);
            response.insert("uuid", uuid);
            response.insert("expr", expr);

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
