use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthCredentials {
  pub client_id: String,
  pub client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthTokenReponse {
  pub access_token: String,
  pub expires_in: u64,
  pub token_type: String,
  pub id_token: String,
  pub scope: String,
}
