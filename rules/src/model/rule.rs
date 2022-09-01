use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Rule {
    pub pk: String,
    pub sk: String,
    pub id: String,
    pub expr: String,
}

impl Rule {
    pub fn new(format: &str, tenant_id: &str, id: &str, expr: &str) -> Rule {
        let sk = Uuid::new_v4().to_string();
        Rule {
            pk: format!("{}#{}", format, tenant_id),
            sk,
            id: String::from(id),
            expr: String::from(expr),
        }
    }

    pub async fn get(
        format: &str,
        uuid: Uuid,
        tenant_id: &str,
        client: &State<Client>,
        table_name: &str,
    ) -> Result<Rule, String> {
        let pk = AttributeValue::S(format!("{}#{}", format, tenant_id));
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
                    id: rule.get("id").unwrap().as_s().unwrap().to_string(),
                    expr: rule.get("expr").unwrap().as_s().unwrap().to_string(),
                }),
                None => Err(String::from("Not Found")),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn create(
        item: &Rule,
        client: &State<Client>,
        table_name: &str,
    ) -> Result<String, String> {
        let response = client
            .put_item()
            .table_name(table_name)
            .item("pk", AttributeValue::S(item.pk.clone()))
            .item("sk", AttributeValue::S(item.sk.clone()))
            .item("id", AttributeValue::S(item.id.clone()))
            .item("expr", AttributeValue::S(item.expr.clone()))
            .condition_expression("attribute_not_exists(pk) AND attribute_not_exists(id)")
            .send()
            .await;

        match response {
            Ok(_) => Ok(item.sk.clone()),
            Err(error) => Err(error.to_string()),
        }
    }
}
