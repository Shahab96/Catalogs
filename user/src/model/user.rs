use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::output::PutItemOutput;
use rocket::State;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::utils::hash_password;

#[derive(Debug, Serialize)]
pub struct User {
    pub pk: String,
    pub gsi_uuid: String,
    pub uuid: String,
    pub email: String,
    pub hashed_password: String,
    pub tenant_list: Vec<String>,
    pub active_tenant: String,
}

impl User {
    pub fn new(email: &str, password: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let pk = format!("USER#{}", email);
        let gsi_uuid = format!("UUID#{}", uuid);
        let email = String::from(email);
        let hashed_password = hash_password(password);
        let mut tenant_list = Vec::new();
        tenant_list.push(Uuid::new_v4().to_string());
        let active_tenant = String::from("");

        Self {
            pk,
            gsi_uuid,
            uuid,
            hashed_password,
            tenant_list,
            active_tenant,
            email: String::from(email),
        }
    }

    pub async fn create(
        user: &Self,
        client: &State<Client>,
        table_name: &str,
    ) -> Result<PutItemOutput, aws_sdk_dynamodb::Error> {
        let result = client
            .put_item()
            .table_name(table_name)
            .item("pk", AttributeValue::S(user.pk.clone()))
            .item("gsi_uuid", AttributeValue::S(user.gsi_uuid.clone()))
            .item("uuid", AttributeValue::S(user.uuid.to_string()))
            .item("email", AttributeValue::S(user.email.clone()))
            .item(
                "hashed_password",
                AttributeValue::S(user.hashed_password.clone()),
            )
            .item("tenant_list", AttributeValue::Ss(user.tenant_list.clone()))
            .item(
                "active_tenant",
                AttributeValue::S(user.active_tenant.clone()),
            )
            .condition_expression("attribute_not_exists(email)")
            .send()
            .await?;

        Ok(result)
    }
}
