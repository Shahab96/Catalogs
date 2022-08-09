use std::error::Error;
use std::collections::HashMap;
use regex::{Regex, Captures};

pub struct Parser {
    rule: String,
    patterns: HashMap<String, String>
}

impl Parser {
    pub fn new(rule: String) -> Self {
        Self{
            rule,
            patterns: HashMap::new()
        }
    }

    pub fn parse(&mut self, log: String) -> Result<String, Box<dyn Error>> {
        
        self.rule = self.grok_to_regex(self.rule.clone());
        let result = self.match_against(log);

        Ok(result)
    }

    fn grok_to_regex(&mut self, grok: String) -> String {
        self.patterns.insert("word".to_string(), r"\b\w+\b".to_string());
        self.patterns.insert("int".to_string(), r"\b\d+\b".to_string());
        self.patterns.insert("uuid".to_string(), "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}".to_string());
        
        let rule;
        let re: Regex = Regex::new(r"(%\{(\w+):(\w+)(?::\w+)?\})").unwrap();

        rule = re.replace_all(grok.as_str(), |caps: &Captures| {

            let pattern: String = match self.patterns.get(&caps[2]) {
                Some(pat) => pat.to_string(),
                None => String::new()
            };

            format!(r"(?P<{}>{})", &caps[3], pattern.as_str())
        });

        (&*rule).to_string()
        
    }

    fn match_against(&self, log: String) -> String{
        
        let mut map: HashMap<String, String> = HashMap::new();
        let re = Regex::new(self.rule.as_str()).unwrap();

        let captures = re.captures(log.as_str()).unwrap();

        for name in re.capture_names() {
            let key = name.unwrap_or("");

            if let "" = key {
                continue;
            }

            let value = &captures[key];
            // println!("{} - {:?}", key, value);

            map.insert(key.to_string(), value.to_string());
        }

        let result = serde_json::to_string(&map).unwrap();
        result
    }
}