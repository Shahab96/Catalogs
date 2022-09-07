use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::response::status::Custom;
use rocket::{get, State};

use crate::model::state;
use crate::service::oauth::login_with_google;

#[get("/oauth2/login?<provider>")]
pub async fn oauth_login(provider: &str, state: &State<state::State>) -> Result<RawHtml<String>, Custom<&'static str>> {
  match provider {
    "google" => {
      if let Ok(response) = login_with_google(&state.google_oauth_credentials).await {
        Ok(RawHtml(response))
      } else {
        Err(Custom(Status::InternalServerError, "Error logging in with Google."))
      }
    },
    _ => Err(Custom(Status::NotImplemented, "Provider not implemented.")),
  }
}
