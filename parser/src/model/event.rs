use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct EvaluationEvent {
    pub rule: String,
    pub sample: String,
}
