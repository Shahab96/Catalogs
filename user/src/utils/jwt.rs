use jwt_simple::prelude::{Claims, Duration, RSAKeyPairLike};
use jwt_simple::algorithms::RS512KeyPair;
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
