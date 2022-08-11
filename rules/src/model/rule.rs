use std::collections::{hash_map::RandomState, HashMap};

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Rule {
    pk: String,
    sk: String,
    uuid: String,
    name: String,
    owner: String,
    rule: String,
}

impl Rule {
    pub fn new(name: String, owner: String, rule: String) -> Rule {
        let uuid = Uuid::new_v4().to_string();
        Rule {
            pk: format!("RULE-{}", owner),
            sk: uuid.clone(),
            uuid,
            name,
            owner,
            rule,
        }
    }

    pub fn new_from_dynamo(rule: &HashMap<String, AttributeValue, RandomState>) -> Rule {
        Rule {
            pk: rule.get("pk").unwrap().as_s().unwrap().to_string(),
            sk: rule.get("sk").unwrap().as_s().unwrap().to_string(),
            uuid: rule.get("uuid").unwrap().as_s().unwrap().to_string(),
            name: rule.get("name").unwrap().as_s().unwrap().to_string(),
            owner: rule.get("owner").unwrap().as_s().unwrap().to_string(),
            rule: rule.get("rule").unwrap().as_s().unwrap().to_string(),
        }
    }
}
