pub struct State {
    pub dynamo: aws_sdk_dynamodb::Client,
    pub table_name: String,
    pub rsa_key: String,
}
