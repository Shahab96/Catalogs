use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use aws_sdk_dynamodb::Error;
use rocket::State;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::model::state;
use crate::model::tenant::Tenant;
use crate::utils::password::hash_password;

#[derive(Debug, Serialize)]
pub struct User {
    pub pk: String,
    pub sk: String,
    pub uuid: String,
    pub email: String,
    pub hashed_password: Option<String>,
    pub tenant_list: Vec<String>,
    pub active_tenant: String,
    pub roles: HashMap<String, Vec<String>>,
}

struct MarshalledUser {
    pk: AttributeValue,
    sk: AttributeValue,
    uuid: AttributeValue,
    email: AttributeValue,
    hashed_password: AttributeValue,
    tenant_list: AttributeValue,
    active_tenant: AttributeValue,
    roles: AttributeValue,
}

impl MarshalledUser {
    pub fn new(user: &User) -> MarshalledUser {
        let pk = AttributeValue::S(format!("USER#{}", user.email));
        let sk = AttributeValue::S(format!("UUID#{}", user.uuid));
        let uuid = AttributeValue::S(user.uuid.to_owned());
        let email = AttributeValue::S(user.email.to_owned());
        let hashed_password = AttributeValue::S(
            user.hashed_password
                .as_ref()
                .unwrap_or(&String::from(""))
                .to_owned(),
        );
        let tenant_list = AttributeValue::Ss(user.tenant_list.to_owned());
        let active_tenant = AttributeValue::S(user.active_tenant.to_owned());
        let mut map: HashMap<String, AttributeValue> = HashMap::new();

        for (tenant_id, role_uuids) in user.roles.iter() {
            let role_vec = AttributeValue::Ss(role_uuids.to_owned());
            map.insert(tenant_id.to_owned(), role_vec.to_owned());
        }

        let roles = AttributeValue::M(map.to_owned());

        MarshalledUser {
            pk,
            sk,
            uuid,
            email,
            hashed_password,
            tenant_list,
            active_tenant,
            roles,
        }
    }

    pub async fn save<'a>(
        &'a self,
        state: &State<state::State>,
    ) -> Result<&'a MarshalledUser, Error> {
        state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", self.pk.to_owned())
            .item("sk", self.sk.to_owned())
            .item("uuid", self.uuid.to_owned())
            .item("email", self.email.to_owned())
            .item("hashed_password", self.hashed_password.to_owned())
            .item("tenant_list", self.tenant_list.to_owned())
            .item("active_tenant", self.active_tenant.to_owned())
            .item("roles", self.roles.to_owned())
            .condition_expression("attribute_not_exists(email)")
            .send()
            .await?;

        Ok(self)
    }

    pub async fn fetch<'a>(
        email: &'a str,
        state: &State<state::State>,
    ) -> Result<MarshalledUser, Error> {
        let marshalled_user = state
            .dynamo
            .get_item()
            .table_name(&state.table_name)
            .key("pk", AttributeValue::S(format!("USER#{}", email)))
            .send()
            .await?;

        if let Some(marshalled_user) = marshalled_user.item() {
            Ok(MarshalledUser {
                pk: marshalled_user.get("pk").unwrap().to_owned(),
                sk: marshalled_user.get("sk").unwrap().to_owned(),
                uuid: marshalled_user.get("uuid").unwrap().to_owned(),
                email: marshalled_user.get("email").unwrap().to_owned(),
                hashed_password: marshalled_user.get("hashed_password").unwrap().to_owned(),
                tenant_list: marshalled_user.get("tenant_list").unwrap().to_owned(),
                active_tenant: marshalled_user.get("active_tenant").unwrap().to_owned(),
                roles: marshalled_user.get("roles").unwrap().to_owned(),
            })
        } else {
            unreachable!("If you're seeing this message, you fucked up. An attempt was made to read a non-existant user from dynamo.")
        }
    }

    pub fn unmarshall<'a>(&'a self) -> Result<User, Error> {
        let pk = self.pk.as_s().unwrap().to_owned();
        let sk = self.sk.as_s().unwrap().to_owned();
        let uuid = self.uuid.as_s().unwrap().to_owned();
        let email = self.email.as_s().unwrap().to_owned();
        let hashed_password = self.hashed_password.as_s();
        let tenant_list = self.tenant_list.as_ss().unwrap().to_owned();
        let active_tenant = self.tenant_list.as_s().unwrap().to_owned();
        let roles = self.roles.as_m().unwrap().to_owned();

        let mut roles_map: HashMap<String, Vec<String>> = HashMap::new();
        let h_pw: Option<String> = None;

        if let Ok(hashed_password) = hashed_password {
            h_pw = Some(hashed_password.to_owned())
        } else {
            h_pw = None;
        }

        for (tenant_id, roles) in roles.iter() {
            roles_map.insert(*tenant_id, *roles.as_ss().unwrap());
        }

        Ok(User {
            pk,
            sk,
            uuid,
            email,
            hashed_password: h_pw,
            tenant_list,
            active_tenant,
            roles: roles_map,
        })
    }
}

impl User {
    fn marshall<'a>(&'a self) -> MarshalledUser {
        MarshalledUser::new(self)
    }

    pub fn new(email: &str, password: Option<&str>) -> User {
        let uuid = Uuid::new_v4().to_string();
        let pk = format!("USER#{}", email);
        let sk = format!("UUID#{}", uuid);
        let email = String::from(email);
        let hashed_password: Option<String>;
        let mut tenant_list = Vec::new();
        let active_tenant = String::from("");
        let roles = HashMap::new();

        let tenant_id = Uuid::new_v4().to_string();
        let tenant = Tenant::new(email, &tenant_id);
        tenant_list.push(tenant_id);

        if let Some(pw) = password {
            hashed_password = Some(hash_password(pw));
        } else {
            hashed_password = None;
        }

        User {
            pk,
            sk,
            uuid,
            hashed_password,
            tenant_list,
            active_tenant,
            roles,
            email,
        }
    }

    pub async fn save<'a>(
        &'a mut self,
        state: &State<state::State>,
    ) -> Result<&'a mut User, Error> {
        let marshalled_user = self.marshall().save(state).await?;

        Ok(self)
    }

    pub async fn get(email: &str, state: &State<state::State>) -> Result<User, Error> {
        MarshalledUser::fetch(email, state).await?.unmarshall()
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
            .expression_attribute_values(
                ":email",
                AttributeValue::S(format!("EMAIL#{}", self.email)),
            )
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
