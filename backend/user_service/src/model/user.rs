use aws_sdk_dynamodb::Error;
use rocket::State;
use serde::Serialize;

use crate::model::marshalled_user::MarshalledUser;
use crate::model::state;

#[derive(Debug, Serialize)]
pub struct User {
    pub email: String,
    pub tenant: String,
    pub hashed_password: String,
    pub roles: Vec<String>,
}

impl User {
    fn marshall<'a>(&'a self) -> MarshalledUser {
        MarshalledUser::new(self)
    }

    pub fn new(email: &str, hashed_password: &str) -> Self {
        let hashed_password = hashed_password.to_owned();
        let email = email.to_owned();
        let roles = vec![String::from("admin"), String::from("root")];
        let tenant = email.to_owned();

        Self {
            email,
            tenant,
            hashed_password,
            roles,
        }
    }

    pub async fn save(&self, state: &State<state::State>) -> bool {
        self.marshall().save(state).await
    }

    pub async fn fetch(email: &str, state: &State<state::State>) -> Result<Option<Self>, Error> {
        let marshalled_user = MarshalledUser::fetch(email, state).await?;

        match marshalled_user {
            Some(m) => Ok(Some(m.unmarshall()?)),
            None => Ok(None),
        }
    }

    pub fn login<'a>(&'a mut self) -> &'a mut Self {
        self.tenant = self.email.to_owned();

        self
    }

    pub fn update_roles<'a>(&'a mut self, roles: &Vec<String>) -> &'a mut Self {
        self.roles = roles.to_owned();

        self
    }
}
