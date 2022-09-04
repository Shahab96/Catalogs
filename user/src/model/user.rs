use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use aws_sdk_dynamodb::output::{PutItemOutput, UpdateItemOutput};
use rocket::State;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::utils::hash_password;
use crate::model::state;

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
        let hashed_password = hash_password(password).to_string();
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
        state: &State<state::State>,
    ) -> Result<PutItemOutput, aws_sdk_dynamodb::Error> {
        let result = state.dynamo
            .put_item()
            .table_name(&state.table_name)
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

    pub async fn login(
        email: &str,
        state: &State<state::State>,
    ) -> Result<UpdateItemOutput, aws_sdk_dynamodb::Error> {
        let user_pk = format!("USER#{}", email);

        let tenant = state.dynamo
            .query()
            .table_name(&state.table_name)
            .index_name("email-index")
            .key_condition_expression("gsi_email = :email")
            .expression_attribute_values(":email", AttributeValue::S(format!("EMAIL#{}", email)))
            .send()
            .await?;

        let tenant_id = match tenant.items() {
            Some(t) => t[0].get("uuid").unwrap().as_s().unwrap().to_string(),
            None => String::from(""),
        };

        let response = state.dynamo
            .update_item()
            .table_name(&state.table_name)
            .key("pk", AttributeValue::S(user_pk))
            .update_expression("SET active_tenant = :tenant_id")
            .expression_attribute_values(":tenant_id", AttributeValue::S(tenant_id))
            .return_values(ReturnValue::AllNew)
            .send()
            .await?;

        Ok(response)
    }
}
