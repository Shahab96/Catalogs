use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Error;
use rocket::State;

use crate::model::state;
use crate::model::user::User;

#[derive(Debug)]
pub struct MarshalledUser {
    pk: AttributeValue,
    sk: AttributeValue,
    email: AttributeValue,
    hashed_password: AttributeValue,
    roles: AttributeValue,
    active: AttributeValue,
}

impl MarshalledUser {
    pub fn new(user: &User) -> MarshalledUser {
        let pk = AttributeValue::S(format!("U#{}", user.email));
        let sk = AttributeValue::S(format!("T#{}", user.email));
        let email = AttributeValue::S(user.email.to_owned());
        let hashed_password = AttributeValue::S(user.hashed_password.to_owned());
        let roles = AttributeValue::Ss(user.roles.to_owned());
        let active = AttributeValue::Bool(user.active);

        MarshalledUser {
            pk,
            sk,
            email,
            hashed_password,
            roles,
            active,
        }
    }

    pub async fn save(&self, state: &State<state::State>) -> Result<(), Error> {
        state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", self.pk.to_owned())
            .item("sk", self.sk.to_owned())
            .item("email", self.email.to_owned())
            .item("hashed_password", self.hashed_password.to_owned())
            .item("roles", self.roles.to_owned())
            .item("active", self.active.to_owned())
            .condition_expression("attribute_not_exists(pk) and attribute_not_exists(sk)")
            .send()
            .await?;

        Ok(())
    }

    pub async fn fetch(
        email: &str,
        state: &State<state::State>,
    ) -> Result<Option<MarshalledUser>, Error> {
        let key = AttributeValue::S(format!("U#{}", email));
        let marshalled_user = state
            .dynamo
            .get_item()
            .table_name(&state.table_name)
            .key("pk", key.clone())
            .key("sk", key)
            .send()
            .await?;

        if let Some(marshalled_user) = marshalled_user.item() {
            Ok(Some(MarshalledUser {
                pk: marshalled_user.get("pk").unwrap().to_owned(),
                sk: marshalled_user.get("sk").unwrap().to_owned(),
                email: marshalled_user.get("email").unwrap().to_owned(),
                hashed_password: marshalled_user.get("hashed_password").unwrap().to_owned(),
                roles: marshalled_user.get("roles").unwrap().to_owned(),
                active: marshalled_user.get("active").unwrap().to_owned(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn unmarshall(&self) -> Result<User, Error> {
        let pk = self.pk.as_s().unwrap().to_owned();
        let sk = self.sk.as_s().unwrap().to_owned();
        let email = self.email.as_s().unwrap().to_owned();
        let hashed_password = self.hashed_password.as_s().unwrap().to_owned();
        let roles = self.roles.as_ss().unwrap().to_owned();
        let active = self.active.as_bool().unwrap().to_owned();

        Ok(User {
            pk,
            sk,
            email,
            hashed_password,
            roles,
            active,
        })
    }
}
