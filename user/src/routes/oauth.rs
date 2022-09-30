use reqwest::ClientBuilder;
use rocket::http::Status;
use rocket::response::status::{Accepted, Custom};
use rocket::response::Redirect;
use rocket::{get, State};
use std::time::SystemTime;
use urlencoding::encode;

use crate::model::oauth::GoogleOAuthTokenReponse;
use crate::model::state;
use crate::model::user::User;
use crate::model::tenant::Tenant;
use crate::utils::jwt::{mint_rsa, verify_rsa};

#[get("/oauth2/login?<provider>")]
pub async fn oauth_login<'a>(
    provider: &'a str,
    state: &'a State<state::State>,
) -> Result<Redirect, Custom<&'static str>> {
    match provider {
        "google" => {
            const GOOGLE_OAUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
            let redirect_url = format!(
                "{}?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email&nonce={}&state=google",
                GOOGLE_OAUTH_URL,
                state.google_oauth_credentials.client_id,
                encode(state.oauth_redirect_uri.as_str()),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            Ok(Redirect::to(redirect_url))
        }
        _ => Err(Custom(Status::NotImplemented, "Provider not implemented.")),
    }
}

#[get("/oauth2/authorization?<code>&<state>&<error>")]
pub async fn oauth_authorization<'a>(
    code: Option<&'a str>,
    state: Option<&'a str>,
    error: Option<&'a str>,
    app_state: &State<state::State>,
) -> Result<Accepted<String>, Custom<&'a str>> {
    if let Some(error) = error {
        return Err(Custom(Status::BadRequest, error));
    }

    let client = ClientBuilder::new().build().unwrap();

    match state.unwrap() {
        "google" => {
            const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
            const GOOGLE_JWKS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
            let response = client
                .post(GOOGLE_TOKEN_URL)
                .form(&[
                    ("code", code.unwrap()),
                    (
                        "client_id",
                        app_state.google_oauth_credentials.client_id.as_str(),
                    ),
                    (
                        "client_secret",
                        app_state.google_oauth_credentials.client_secret.as_str(),
                    ),
                    ("redirect_uri", app_state.oauth_redirect_uri.as_str()),
                    ("grant_type", "authorization_code"),
                ])
                .send()
                .await
                .unwrap();

            let response_json: GoogleOAuthTokenReponse =
                serde_json::from_str(response.text().await.unwrap().as_str()).unwrap();
            let email_raw = verify_rsa(GOOGLE_JWKS_URL, response_json.id_token.as_str())
                .await
                .unwrap();

            let email = &email_raw[1..email_raw.len() - 1];

            match User::get(email, &app_state).await {
                Ok(result) => {
                    let mut user = User::new(email, None);

                    if result.is_none() {
                        User::create(&user, app_state).await.unwrap();
                        let tenant = Tenant::new(&user.email, &user.tenant_list.first().unwrap());
                        Tenant::create(&tenant, app_state).await.unwrap();
                    }

                    User::login(&mut user, app_state).await.unwrap();

                    let token = mint_rsa(&app_state.rsa_key, email, &user.active_tenant).unwrap();

                    Ok(Accepted(Some(token)))
                }
                Err(e) => {
                    println!("{}", e);
                    Err(Custom(Status::InternalServerError, "There was an error."))
                },
            }
        }
        _ => Err(Custom(Status::NotImplemented, "Provider not implemented.")),
    }
}