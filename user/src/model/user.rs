use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use rocket::State;
use serde::Serialize;
use uuid::Uuid;

use crate::model::state;
use crate::utils::password::hash_password;

#[derive(Debug, Serialize)]
pub struct User {
    pub pk: String,
    pub gsi_uuid: String,
    pub uuid: String,
    pub email: String,
    pub hashed_password: Option<String>,
    pub tenant_list: Vec<String>,
    pub active_tenant: String,
}

impl User {
    pub fn new(email: &str, password: Option<&str>) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let pk = format!("USER#{}", email);
        let gsi_uuid = format!("UUID#{}", uuid);
        let email = String::from(email);
        let hashed_password: Option<String>;
        let mut tenant_list = Vec::new();
        let active_tenant = String::from("");

        tenant_list.push(Uuid::new_v4().to_string());

        if let Some(pw) = password {
            hashed_password = Some(hash_password(pw));
        } else {
            hashed_password = None;
        }

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

    pub async fn get(
        email: &str,
        state: &State<state::State>,
    ) -> Result<Option<User>, aws_sdk_dynamodb::Error> {
        let pk = format!("USER#{}", email);
        let result = state
            .dynamo
            .get_item()
            .table_name(&state.table_name)
            .key("pk", AttributeValue::S(pk))
            .send()
            .await?;

        if let Some(item) = result.item() {
            Ok(Some(User {
                pk: item.get("pk").unwrap().as_s().unwrap().to_string(),
                gsi_uuid: item.get("gsi_uuid").unwrap().as_s().unwrap().to_string(),
                uuid: item.get("uuid").unwrap().as_s().unwrap().to_string(),
                email: item.get("email").unwrap().as_s().unwrap().to_string(),
                hashed_password: Some(
                    item.get("hashed_password")
                        .unwrap()
                        .as_s()
                        .unwrap()
                        .to_string(),
                ),
                tenant_list: item.get("tenant_list").unwrap().as_ss().unwrap().clone(),
                active_tenant: item
                    .get("active_tenant")
                    .unwrap()
                    .as_s()
                    .unwrap()
                    .to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create(
        &self,
        state: &State<state::State>,
    ) -> Result<(), aws_sdk_dynamodb::Error> {
        state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", AttributeValue::S(self.pk.clone()))
            .item("gsi_uuid", AttributeValue::S(self.gsi_uuid.clone()))
            .item("uuid", AttributeValue::S(self.uuid.to_string()))
            .item("email", AttributeValue::S(self.email.clone()))
            .item(
                "hashed_password",
                AttributeValue::S(self.hashed_password.as_ref().unwrap_or(&String::from("")).clone()),
            )
            .item("tenant_list", AttributeValue::Ss(self.tenant_list.clone()))
            .item(
                "active_tenant",
                AttributeValue::S(self.active_tenant.clone()),
            )
            .condition_expression("attribute_not_exists(email)")
            .send()
            .await?;

        Ok(())
    }

    pub async fn login(
        &mut self,
        state: &State<state::State>,
    ) -> Result<(), aws_sdk_dynamodb::Error> {
        let user_pk = format!("USER#{}", self.email);
        let tenant = state
            .dynamo
            .query()
            .table_name(&state.table_name)
            .index_name("email-index")
            .key_condition_expression("gsi_email = :email")
            .expression_attribute_values(":email", AttributeValue::S(format!("EMAIL#{}", self.email)))
            .send()
            .await?;

        let tenant_id = match tenant.items() {
            Some(t) => t[0].get("uuid").unwrap().as_s().unwrap().to_string(),
            None => String::from(""),
        };

        state
            .dynamo
            .update_item()
            .table_name(&state.table_name)
            .key("pk", AttributeValue::S(user_pk))
            .update_expression("SET active_tenant = :tenant_id")
            .expression_attribute_values(":tenant_id", AttributeValue::S(tenant_id.clone()))
            .return_values(ReturnValue::AllNew)
            .send()
            .await?;

        self.active_tenant = tenant_id;
        Ok(())
    }
}
