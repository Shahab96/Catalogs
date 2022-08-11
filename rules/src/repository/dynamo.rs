use crate::model::rule::Rule;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use log::error;

pub struct Dynamo {
    client: Client,
    table_name: String,
}

impl Dynamo {
    pub fn init(table_name: String, config: SdkConfig) -> Dynamo {
        let client = Client::new(&config);
        Dynamo { table_name, client }
    }

    pub async fn get_rule(&self, uuid: String, owner: String) -> Option<Rule> {
        let pk = AttributeValue::S(format!("RULE-{}", owner));
        let sk = AttributeValue::S(uuid);

        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", pk)
            .key("sk", sk)
            .send()
            .await;

        return match result {
            Ok(output) => {
                let rule = output.item()?;
                Some(Rule::new_from_dynamo(&rule))
            }
            Err(error) => {
                error!("{:?}", error);
                None
            }
        };
    }
}
