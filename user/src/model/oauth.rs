use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthCredentials {
  pub client_id: String,
  pub client_secret: String,
}
