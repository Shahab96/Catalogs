use aws_sdk_dynamodb::Error::ConditionalCheckFailedException;
use rocket::http::Status;
use rocket::response::status::{Accepted, Created, Custom};
use rocket::serde::json::Json;
use rocket::{post, State};
use serde::Deserialize;

// use crate::guards::user::AuthenticatedUser;
// use crate::model::role::Role;
use crate::model::state;
use crate::model::tenant::Tenant;
use crate::model::user::User;
use crate::utils::jwt::mint_rsa;
use crate::utils::password::verify_password;

#[derive(Deserialize)]
pub struct ClientRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
pub struct AddRolesRequest {
    roles: Vec<String>,
}

#[post("/register", data = "<data>", format = "json")]
pub async fn register(
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Result<Created<()>, Custom<&'static str>> {
    let mut user = User::new(data.email, Some(data.password));

    match user.save(state).await {
        Ok(user) => {
            let tenant = Tenant::new(&user.email, &user.tenant_list.first().unwrap());
            match Tenant::create(&tenant, state).await {
                Ok(_) => Ok(Created::new("/")),
                Err(_) => Err(Custom(
                    Status::InternalServerError,
                    "Error creating your account.",
                )),
            }
        }
        Err(err) => match err {
            ConditionalCheckFailedException(_) => {
                Err(Custom(Status::Conflict, "User already exists."))
            }
            _ => Err(Custom(Status::InternalServerError, "Internal Server Error")),
        },
    }
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login<'a>(
    state: &State<state::State>,
    data: Json<ClientRequest<'_>>,
) -> Result<Accepted<String>, Custom<&'a str>> {
    let mut user = User::new(data.email, Some(data.password));
    let result = User::login(&mut user, state).await;

    match result {
        Ok(()) => {
            if verify_password(data.password) {
                let tenant_id = &user.active_tenant;
                let jwt = mint_rsa(&state.rsa_key, &data.email, &tenant_id).unwrap();

                Ok(Accepted(Some(String::from(jwt.as_str()))))
            } else {
                Err(Custom(Status::Unauthorized, "Incorrect email or password."))
            }
        }
        Err(err) => match err {
            ConditionalCheckFailedException(_) => {
                Err(Custom(Status::NotFound, "Incorrect email or password."))
            }
            _ => Err(Custom(Status::InternalServerError, "Internal Server Error")),
        },
    }
}

// #[post("/addRoles", data = "<data>", format = "json")]
// pub async fn add_roles<'a>(
//     state: &State<state::State>,
//     data: Json<AddRolesRequest>,
//     authenticated_user: AuthenticatedUser<'_>,
// ) -> Custom<&'a str> {
//     let tenant_id = authenticated_user.tenant_id;
//     if let Ok(Some(user)) = User::get(authenticated_user.email, state).await {
//         for role in data.roles.iter() {
//             user.roles.insert(tenant_id)
//         }
//     }

//     Custom(Status::Ok, "Added roles.")
// }
