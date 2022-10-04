use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use rocket::State;
use serde::Serialize;

use crate::model::state;

#[derive(Debug, Serialize)]
pub struct Tenant {
    pub pk: String,
    pub gsi_email: String,
    pub uuid: String,
    pub email: String,
    pub members: Vec<String>,
}

impl Tenant {
    pub fn new(email: &str, tenant_id: &str) -> Self {
        let uuid = String::from(tenant_id);
        let pk = format!("TENANT#{}", uuid);
        let gsi_email = format!("EMAIL#{}", email);
        let email = String::from(email);
        let mut members = Vec::new();
        members.push(email.clone());

        Self {
            pk,
            gsi_email,
            uuid,
            email,
            members,
        }
    }

    pub async fn create(
        tenant: &Self,
        state: &State<state::State>,
    ) -> Result<PutItemOutput, aws_sdk_dynamodb::Error> {
        let result = state.dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", AttributeValue::S(tenant.pk.clone()))
            .item("gsi_email", AttributeValue::S(tenant.gsi_email.clone()))
            .item("uuid", AttributeValue::S(tenant.uuid.to_string()))
            .item("email", AttributeValue::S(tenant.email.clone()))
            .item("members", AttributeValue::Ss(tenant.members.clone()))
            .send()
            .await?;

        Ok(result)
    }
}