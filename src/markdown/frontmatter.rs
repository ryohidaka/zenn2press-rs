use std::collections::HashMap;

use serde_yaml::Value;

/// Type alias for the configuration parsed from the frontmatter.
/// This is a mapping of string keys to YAML values.
type FrontmatterConfig = HashMap<String, Value>;

/// Parses the frontmatter and content from a given string.
///
/// The input string is expected to contain a frontmatter section
/// enclosed by "---" at the beginning and end, followed by the actual content.
///
/// # Arguments
///
/// * `content` - A string slice that contains the frontmatter and content.
///
/// # Returns
///
/// A `Result` containing a tuple with the parsed content as a string and the
/// frontmatter as a `FrontmatterConfig` (i.e., a `HashMap<String, Value>`).
/// If parsing fails, an error is returned.
///
/// # Errors
///
/// This function will return an error if the YAML frontmatter cannot be parsed.
pub fn parse_frontmatter(
    content: &str,
) -> Result<(String, FrontmatterConfig), Box<dyn std::error::Error>> {
    // Split the input string into three parts: the opening separator, the frontmatter, and the content
    let mut parts = content.splitn(3, "---");

    // Skip the first part (the opening separator)
    parts.next();

    // The second part is the frontmatter, which is optional and needs to be trimmed
    let frontmatter = parts.next().unwrap_or("").trim();

    // The third part is the content, which is also optional and needs to be trimmed
    let content = parts.next().unwrap_or("").trim();

    // Parse the frontmatter into a HashMap using Serde YAML
    let data: FrontmatterConfig = serde_yaml::from_str(frontmatter)?;

    // Return the content and the parsed frontmatter as a tuple
    Ok((content.to_string(), data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_with_valid_input() {
        let input = "---\ntitle: Example\n---\nThis is the content.";
        let (content, config) = parse_frontmatter(input).unwrap();

        assert_eq!(content, "This is the content.");
        assert_eq!(config.get("title").unwrap(), "Example");
    }

    #[test]
    fn test_parse_frontmatter_with_empty_frontmatter() {
        let input = "---\n---\nThis is the content.";
        let (content, config) = parse_frontmatter(input).unwrap();

        assert_eq!(content, "This is the content.");
        assert!(config.is_empty());
    }

    #[test]
    fn test_parse_frontmatter_with_invalid_yaml() {
        let input = "---\ntitle: [Invalid YAML\n---\nThis is the content.";
        let result = parse_frontmatter(input);

        assert!(result.is_err());
    }
}
