use std::fs;

use crate::parser::types::{IProtoweldParser, ProtoweldParser};

impl IProtoweldParser for ProtoweldParser {
    fn parse(filename: &str) -> Result<ProtoweldParser, String> {
        let content = fs::read_to_string(filename);
        if let Err(error) = content {
            return Err(error.to_string());
        }

        let yaml_result = serde_yaml::from_str::<ProtoweldParser>(&content.unwrap());
        if let Err(error) = yaml_result {
            return Err(error.to_string());
        }

        Ok(yaml_result.unwrap())
    }
}
