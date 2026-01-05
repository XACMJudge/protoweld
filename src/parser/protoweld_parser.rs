//! Implementation of the Protoweld parser for YAML configuration files.

use std::fs;

use crate::parser::types::{IProtoweldParser, ProtoweldParser};

impl IProtoweldParser for ProtoweldParser {
    /// Parses a YAML configuration file into a `ProtoweldParser` structure.
    ///
    /// This implementation reads the file from disk, deserializes it from YAML format,
    /// and validates the structure. The file should contain an `active_projects` array
    /// with project configurations.
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to the YAML configuration file
    ///
    /// # Returns
    ///
    /// * `Ok(ProtoweldParser)` - Successfully parsed configuration
    /// * `Err(String)` - Error message if file reading or YAML parsing fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be read (file not found, permission denied, etc.)
    /// - The YAML content is malformed or doesn't match the expected structure
    /// - Required fields are missing from the configuration
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
