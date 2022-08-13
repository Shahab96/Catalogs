use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Rule {
    pk: String,
    sk: String,
    uuid: String,
    name: String,
    owner: String,
    rule: String,
}

impl Rule {
    pub fn new(name: &str, owner: &str, rule: &str) -> Rule {
        let uuid = Uuid::new_v4().to_string();
        Rule {
            pk: format!("RULE#{}", owner),
            sk: uuid.clone(),
            uuid,
            name: String::from(name),
            owner: String::from(owner),
            rule: String::from(rule),
        }
    }

    pub async fn get(uuid: Uuid, owner: &str) -> Result<Rule, String> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        match std::env::var("TABLE_NAME") {
            Ok(table_name) => {
                let pk = AttributeValue::S(format!("RULE#{}", owner));
                let sk = AttributeValue::S(uuid.to_string());

                let response = client
                    .get_item()
                    .table_name(table_name)
                    .key("pk", pk)
                    .key("sk", sk)
                    .send()
                    .await;

                match response {
                    Ok(response_data) => match response_data.item() {
                        Some(rule) => Ok(Rule {
                            pk: rule.get("pk").unwrap().as_s().unwrap().to_string(),
                            sk: rule.get("sk").unwrap().as_s().unwrap().to_string(),
                            uuid: rule.get("uuid").unwrap().as_s().unwrap().to_string(),
                            name: rule.get("name").unwrap().as_s().unwrap().to_string(),
                            owner: rule.get("owner").unwrap().as_s().unwrap().to_string(),
                            rule: rule.get("rule").unwrap().as_s().unwrap().to_string(),
                        }),
                        None => Err(String::from("Not Found")),
                    },
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
