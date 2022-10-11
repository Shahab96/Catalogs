use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Error;
use rocket::State;

use crate::model::state;
use crate::model::user::User;

#[derive(Debug)]
pub struct MarshalledUser {
    pk: AttributeValue,
    sk: AttributeValue,
    hashed_password: AttributeValue,
    roles: AttributeValue,
}

impl MarshalledUser {
    pub fn new(user: &User) -> MarshalledUser {
        let pk = AttributeValue::S(format!("U#{}", user.email));
        let sk = AttributeValue::S(format!("T#{}", user.email));
        let hashed_password = AttributeValue::S(user.hashed_password.to_owned());
        let roles = AttributeValue::Ss(user.roles.to_owned());

        MarshalledUser {
            pk,
            sk,
            hashed_password,
            roles,
        }
    }

    pub async fn save(&self, state: &State<state::State>) -> bool {
        let result = state
            .dynamo
            .put_item()
            .table_name(&state.table_name)
            .item("pk", self.pk.to_owned())
            .item("sk", self.sk.to_owned())
            .item("hashed_password", self.hashed_password.to_owned())
            .item("roles", self.roles.to_owned())
            .send()
            .await;

        match result {
            Ok(_) => true,
            Err(e) => {
                println!(
                    "If you're seeing this message, you may have fucked up. {:?}",
                    e
                );
                false
            }
        }
    }

    pub async fn fetch(
        email: &str,
        state: &State<state::State>,
    ) -> Result<Option<MarshalledUser>, Error> {
        let pk = AttributeValue::S(format!("U#{}", email));
        let sk = AttributeValue::S(format!("T#{}", email));
        let marshalled_user = state
            .dynamo
            .get_item()
            .table_name(&state.table_name)
            .key("pk", pk)
            .key("sk", sk)
            .send()
            .await?;

        if let Some(marshalled_user) = marshalled_user.item() {
            Ok(Some(MarshalledUser {
                pk: marshalled_user.get("pk").unwrap().to_owned(),
                sk: marshalled_user.get("sk").unwrap().to_owned(),
                hashed_password: marshalled_user.get("hashed_password").unwrap().to_owned(),
                roles: marshalled_user.get("roles").unwrap().to_owned(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn unmarshall(&self) -> Result<User, Error> {
        let email = self
            .pk
            .as_s()
            .unwrap()
            .strip_prefix("U#")
            .unwrap()
            .to_owned();
        let tenant = self
            .sk
            .as_s()
            .unwrap()
            .strip_prefix("T#")
            .unwrap()
            .to_owned();
        let roles = self.roles.as_ss().unwrap().to_owned();
        let hashed_password = self.hashed_password.as_s().unwrap().to_owned();

        Ok(User {
            email,
            hashed_password,
            roles,
            tenant,
        })
    }
}
