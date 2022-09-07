use std::time::SystemTime;

use reqwest::ClientBuilder;
use urlencoding::encode;

use crate::model::oauth::GoogleOAuthCredentials;

pub async fn login_with_google(
    credentials: &GoogleOAuthCredentials,
) -> Result<String, Box<dyn std::error::Error>> {
    const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
    let oauth_redirect_uri = std::env::var("OAUTH_REDIRECT_URI").unwrap();

    let request_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email&nonce={}",
        GOOGLE_AUTH_URL,
        credentials.client_id,
        encode(oauth_redirect_uri.as_str()),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let client = ClientBuilder::new().build()?;
    let response = client.get(request_url).send().await?;

    Ok(response.text_with_charset("utf-8").await?)
}
