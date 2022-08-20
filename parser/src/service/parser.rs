use regex::{Captures, Regex};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;

pub struct Parser {
    rule: String,
    patterns: HashMap<String, String>,
}

impl Parser {
    pub fn new(rule: String) -> Self {
        Self {
            rule,
            patterns: HashMap::new(),
        }
    }

    pub fn parse(&mut self, log: String) -> Result<Map<String, Value>, Box<dyn Error>> {
        self.rule = self.grok_to_regex(self.rule.clone());
        let result = self.match_against(log);

        Ok(result)
    }

    fn grok_to_regex(&mut self, grok: String) -> String {
        self.patterns
            .insert("word".to_string(), r"\b\w+\b".to_string());
        self.patterns
            .insert("int".to_string(), r"\b\d+\b".to_string());
        self.patterns.insert(
            "uuid".to_string(),
            "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}".to_string(),
        );

        let rule;
        let re: Regex = Regex::new(r"(%\{(\w+):(\w+)(?::\w+)?\})").unwrap();

        rule = re.replace_all(grok.as_str(), |caps: &Captures| {
            let pattern: String = match self.patterns.get(&caps[2]) {
                Some(pat) => pat.to_string(),
                None => String::new(),
            };

            format!(r"(?P<{}>{})", &caps[3], pattern.as_str())
        });

        (&*rule).to_string()
    }

    fn match_against(&self, log: String) -> Map<String, Value> {
        let mut map: Map<String, Value> = Map::new();
        let re = Regex::new(self.rule.as_str()).unwrap();

        let captures = re.captures(log.as_str()).unwrap();

        for name in re.capture_names() {
            let key = name.unwrap_or("");

            if let "" = key {
                continue;
            }

            let value = &captures[key];
            // println!("{} - {:?}", key, value);

            map.insert(key.to_string(), Value::from(value));
        }

        return map;
    }
}
