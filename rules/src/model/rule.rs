use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Rule<'r> {
    pub pk: &'r str,
    pub sk: &'r str,
    pub id: Option<&'r str>,
    pub expr: Option<&'r str>,
}

impl Rule<'_> {
    pub fn new<'r>(format: &'r str, tenant_id: &'r str, id: &'r str, expr: &'r str) -> Rule<'r> {
        let sk = Uuid::new_v4().to_string().as_str();
        Rule {
            pk: format!("{}#{}", format, tenant_id).as_str(),
            sk,
            id: Some(id),
            expr: Some(expr),
        }
    }

    pub async fn get<'r>(request: &'r mut Rule<'r>) -> Result<(), String> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        match std::env::var("TABLE_NAME") {
            Ok(table_name) => {
                let pk = AttributeValue::S(String::from(request.pk));
                let sk = AttributeValue::S(String::from(request.sk));

                let response = client
                    .get_item()
                    .table_name(table_name)
                    .key("pk", pk)
                    .key("sk", sk)
                    .send()
                    .await;

                match response {
                    Ok(response_data) => match response_data.item() {
                        Some(rule) => {
                            let id = rule.get("id").unwrap().as_s().unwrap().as_str();
                            let expr = rule.get("expr").unwrap().as_s().unwrap().as_str();
                            request.id = Some(id);
                            request.expr = Some(expr);

                            Ok(())
                        },
                        None => Err(String::from("Not Found")),
                    },
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn put<'r>(item: &'r Rule<'_>) -> Result<&'r str, String> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        match std::env::var("TABLE_NAME") {
            Ok(table_name) => {
                let response = client
                    .put_item()
                    .table_name(table_name)
                    .item("pk", AttributeValue::S(String::from(item.pk)))
                    .item("sk", AttributeValue::S(String::from(item.sk)))
                    .item("id", AttributeValue::S(String::from(item.id)))
                    .item("expr", AttributeValue::S(String::from(item.expr)))
                    .condition_expression("attribute_not_exists(pk) AND attribute_not_exists(id)")
                    .send()
                    .await;

                match response {
                    Ok(_) => Ok(&item.sk),
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
