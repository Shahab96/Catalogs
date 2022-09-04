pub struct State {
    pub dynamo: aws_sdk_dynamodb::Client,
    pub table_name: String,
    pub access_token: String,
    pub refresh_token: String,
}
