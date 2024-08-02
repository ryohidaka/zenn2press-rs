use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

/// `ConfigFile` is a structure representing a configuration file
/// that can contain arbitrary key-value pairs.
/// The `other` field holds a map of configuration options, where keys are strings
/// and values are of type `serde_yaml::Value`.
#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    /// A map to hold arbitrary key-value pairs from the configuration file.
    #[serde(flatten)]
    pub other: std::collections::HashMap<String, serde_yaml::Value>,
}

/// Reads a YAML configuration file from the specified path and returns
/// a `ConfigFile` struct containing the parsed configuration data.
///
/// # Arguments
///
/// * `config_file` - A string slice that holds the path to the configuration file.
///
/// # Returns
///
/// * `Ok(ConfigFile)` if the file is read and parsed successfully.
/// * `Err(Box<dyn Error>)` if there is an error reading the file or parsing the YAML.
pub fn read_config_file(config_file: &str) -> Result<ConfigFile, Box<dyn Error>> {
    // Read the contents of the configuration file into a string
    let config_content = fs::read_to_string(config_file)?;

    // Parse the YAML string into a ConfigFile struct
    let config: ConfigFile = serde_yaml::from_str(&config_content)?;

    // Return the parsed ConfigFile struct
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test case verifies that the `read_config_file` function
    /// correctly reads and parses a YAML configuration file.
    #[test]
    fn test_read_config_file() {
        // Create a sample YAML content
        let yaml_content = r#"
        key1: value1
        key2: value2
        "#;

        // Write the sample YAML content to a temporary file
        let temp_file_path = "test_config.yaml";
        fs::write(temp_file_path, yaml_content).expect("Unable to write test file");

        // Read and parse the configuration file
        let config = read_config_file(temp_file_path).expect("Failed to read config file");

        // Check if the parsed values match the expected values
        assert_eq!(config.other.get("key1").unwrap(), "value1");
        assert_eq!(config.other.get("key2").unwrap(), "value2");

        // Clean up the temporary file
        fs::remove_file(temp_file_path).expect("Unable to delete test file");
    }
}
