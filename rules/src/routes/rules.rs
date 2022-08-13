use crate::guards::api_key::ApiKey;
use crate::model::rule::Rule;

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/rule/<uuid>")]
pub async fn get_rule(api_key: ApiKey<'_>, uuid: Uuid) -> Result<Json<Rule>, Status> {
    let result = Rule::get(uuid, api_key.value).await;

    match result {
        Ok(rule) => Ok(Json(rule)),
        Err(error) => match error.as_str() {
            "Not Found" => Err(Status::NotFound),
            _ => {
                println!("{:?}", error);
                Err(Status::InternalServerError)
            }
        },
    }
}
