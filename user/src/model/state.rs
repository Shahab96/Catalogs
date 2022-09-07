use crate::model::oauth::GoogleOAuthCredentials;

pub struct State {
    pub dynamo: aws_sdk_dynamodb::Client,
    pub table_name: String,
    pub rsa_key: String,
    pub google_oauth_credentials: GoogleOAuthCredentials,
}
