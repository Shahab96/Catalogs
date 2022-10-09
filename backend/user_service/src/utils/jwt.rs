use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use jwt::{Header, Token};
use jwt_simple::algorithms::RS512KeyPair;
use jwt_simple::prelude::{Claims, Duration, RSAKeyPairLike};
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    sub: String,
    tid: String,
}

pub fn mint_rsa<'a>(
    rsa_key: &'a String,
    sub: &'a str,
    tid: &'a str,
) -> Result<String, Box<dyn std::error::Error>> {
    let key_pair = RS512KeyPair::from_pem(&rsa_key.as_str())?;
    let auth_claims = AuthClaims {
        sub: String::from(sub),
        tid: String::from(tid),
    };
    let claims = Claims::with_custom_claims(auth_claims, Duration::from_mins(5))
        .with_issuer(std::env::var("DOMAIN_NAME").unwrap());

    Ok(key_pair.sign(claims)?)
}

pub fn get_claims(token: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let token: Token<Header, AuthClaims, _> = Token::parse_unverified(token)?;
    let claims = token.claims();

    Ok((claims.sub.to_owned(), claims.tid.to_owned()))
}

pub async fn verify_rsa<'a>(
    jwks_url: &'a str,
    token: &'a str,
) -> Result<String, Box<dyn std::error::Error>> {
    let jwks: JWKS = serde_json::from_str(
        ClientBuilder::new()
            .build()?
            .get(jwks_url)
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )?;
    let kid = token_kid(token)?.unwrap();
    let jwk = jwks.find(kid.as_str()).unwrap();

    let validations = vec![Validation::NotExpired, Validation::SubjectPresent];

    let valid_token = validate(token, jwk, validations)?;
    Ok(valid_token.claims.get("email").unwrap().to_string())
}
