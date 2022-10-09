use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Error;
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
    pub fn new(email: &str, tenant_id: &str) -> Tenant {
        let uuid = String::from(tenant_id);
        let pk = format!("TENANT#{}", uuid);
        let gsi_email = format!("EMAIL#{}", email);
        let email = String::from(email);
        let mut members = Vec::new();
        members.push(email.clone());

        Tenant {
            pk,
            gsi_email,
            uuid,
            email,
            members,
        }
    }

    pub async fn save<'a>(
        &'a mut self,
        state: &State<state::State>,
    ) -> Result<&'a mut Tenant, Error> {
        let pk = AttributeValue::S(self.pk.to_owned());
        let gsi_email = AttributeValue::S(self.gsi_email.to_owned());
        let uuid = AttributeValue::S(self.uuid.to_string());
        let email = AttributeValue::S(self.email.to_owned());
        let members = AttributeValue::Ss(self.members.to_owned());

        let result = state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", pk)
            .item("gsi_email", gsi_email)
            .item("uuid", uuid)
            .item("email", email)
            .item("members", members)
            .send()
            .await?;

        Ok(self)
    }
}
