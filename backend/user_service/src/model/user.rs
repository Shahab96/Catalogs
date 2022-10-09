use aws_sdk_dynamodb::Error;
use rocket::State;
use serde::Serialize;

use crate::model::marshalled_user::MarshalledUser;
use crate::model::state;

#[derive(Debug, Serialize)]
pub struct User {
    pub pk: String,
    pub sk: String,
    pub email: String,
    pub active: bool,
    pub hashed_password: String,
    pub roles: Vec<String>,
}

impl User {
    fn marshall<'a>(&'a self) -> MarshalledUser {
        MarshalledUser::new(self)
    }

    pub fn new(email: &str, hashed_password: &str) -> Self {
        let pk = format!("U#{}", email);
        let sk = format!("T#{}", email);
        let hashed_password = hashed_password.to_owned();
        let email = email.to_owned();
        let roles = vec![String::from("admin"), String::from("root")];
        let active = true;

        Self {
            pk,
            sk,
            hashed_password,
            roles,
            email,
            active,
        }
    }

    pub async fn save(&self, state: &State<state::State>) -> Result<(), Error> {
        self.marshall().save(state).await?;

        Ok(())
    }

    pub async fn fetch(email: &str, state: &State<state::State>) -> Result<Option<Self>, Error> {
        let marshalled_user = MarshalledUser::fetch(email, state).await?;

        match marshalled_user {
            Some(m) => Ok(Some(m.unmarshall()?)),
            None => Ok(None),
        }
    }

    pub async fn login<'a>(
        &'a mut self,
        state: &State<state::State>,
    ) -> Result<&'a mut Self, Error> {
        self.active = true;

        self.save(state).await?;

        Ok(self)
    }
}
