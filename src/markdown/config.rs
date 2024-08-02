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
