use crate::repository::dynamo::Dynamo;
use actix_web::error::{Error, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, HttpRequest};
use log::error;

#[get("/rule/{name}")]
pub async fn get_rule(
    data: Data<Dynamo>,
    request: HttpRequest,
    uuid: Path<String>,
) -> Result<Json<String>, Error> {
    let owner: String;
    match request.headers().get("x-api-key") {
        None => ErrorUnauthorized("Unauthorized"),
        Some(apiKey) => match apiKey.to_str() {
            Ok(val) => owner = val.to_string(),
            Err(error) => {
                error!("Error {:?}", error);
                ErrorInternalServerError("Internal Server Error")
            }
        },
    };

    match data.get_rule(uuid, owner) {
        Some(rule) => Json(serde_json::to_string(rule)),
        Err() => ErrorNotFound("Rule not found."),
    }
}
