use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Error;
use rocket::State;
use serde::Serialize;
use uuid::Uuid;

use crate::model::state;

#[derive(Debug, Serialize)]
pub struct Role {
    pub pk: String,
    pub sk: String,
    pub uuid: String,
    pub name: String,
    pub scopes: Vec<String>,
}

impl Role {
    pub fn new(tenant_id: &str, name: &str) -> Role {
        let uuid = Uuid::new_v4().to_string();
        let pk = format!("ROLE#{}", tenant_id);
        let sk = uuid.clone();
        let scopes = Vec::new();

        Role {
            pk,
            sk,
            uuid,
            name: name.to_string(),
            scopes,
        }
    }

    pub async fn get(
        tenant_id: &str,
        role_uuid: &str,
        state: &State<state::State>,
    ) -> Result<Role, Error> {
        let role_uuid = role_uuid.to_string();
        let role = state
            .dynamo
            .get_item()
            .table_name(&state.table_name)
            .key("pk", AttributeValue::S(format!("ROLE#{}", tenant_id)))
            .key("sk", AttributeValue::S(role_uuid.clone()))
            .send()
            .await?;

        if let Some(role) = role.item() {
            Ok(Role {
                pk: role.get("pk").unwrap().as_s().unwrap().clone(),
                sk: role.get("sk").unwrap().as_s().unwrap().clone(),
                uuid: role_uuid,
                name: role.get("name").unwrap().as_s().unwrap().clone(),
                scopes: role.get("scopes").unwrap().as_ss().unwrap().clone(),
            })
        } else {
            unreachable!("If you're seeing this message, you fucked up. A role uuid was passed in that doesn't actually exist. Might wanna go check how that happened.")
        }
    }

    pub async fn save<'a>(
        &'a mut self,
        state: &State<state::State>,
    ) -> Result<&'a mut Role, Error> {
        self.scopes.sort();

        state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", AttributeValue::S(self.pk.clone()))
            .item("sk", AttributeValue::S(self.sk.clone()))
            .item("uuid", AttributeValue::S(self.uuid.to_string()))
            .item("name", AttributeValue::S(self.name.clone()))
            .item("scopes", AttributeValue::Ss(self.scopes.clone()))
            .send()
            .await?;

        Ok(self)
    }

    pub fn add_scope<'a>(&'a mut self, scope: &str) -> &'a mut Role {
        let scope = scope.to_string();
        if !self.scopes.contains(&scope) {
            self.scopes.push(scope);
        }
        self
    }

    pub fn add_scopes<'a>(&'a mut self, scopes: &[&str]) -> &'a mut Role {
        for scope in scopes.iter() {
            self.add_scope(scope);
        }

        self
    }

    pub fn remove_scope<'a>(&'a mut self, scope: &str) -> &'a mut Role {
        let scope = scope.to_string();
        if self.scopes.contains(&scope) {
            let index = self.scopes.iter().position(|r| r == &scope);
            match index {
                Some(index) => {
                    self.scopes.swap_remove(index);
                }
                None => {}
            }
        }

        self
    }

    pub fn remove_scopes<'a>(&'a mut self, scopes: &[&str]) -> &'a mut Role {
        for scope in scopes.iter() {
            self.remove_scope(scope);
        }

        self
    }
}
