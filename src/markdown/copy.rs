use super::frontmatter::parse_frontmatter;

use std::{fs, path::Path};

/// Copies a Markdown file to a destination directory, optionally updating its frontmatter.
///
/// This function reads the content of the specified Markdown file, parses its frontmatter,
/// and optionally merges it with additional configuration data. If the frontmatter does not
/// contain a "title" key, the function will log a message and exit early. Otherwise, it updates
/// the content of the file by adding the title at the beginning and writes the updated file
/// to the destination directory.
///
/// # Arguments
///
/// * `dest_dir` - The destination directory where the file will be copied.
/// * `file` - The path to the Markdown file that needs to be copied.
/// * `frontmatter_config` - Optional configuration data to merge with the file's frontmatter.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error wrapped in `Box` on failure.
pub fn copy_markdown_file(dest_dir: &str, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Read the content of the file
    let file_content = fs::read_to_string(&file)?;

    // Parse the frontmatter of the file
    let (content, data) = parse_frontmatter(&file_content)?;

    // Check if the file has a title in its frontmatter
    if !data.contains_key("title") {
        println!("No title found in the frontmatter of {:?}", file);
        return Ok(());
    }

    // Convert the title to a string
    let title = data["title"].as_str().unwrap_or("Untitled");

    // Update the content of the file by adding the title at the beginning
    let updated_content = format!("# {}\n{}", title, content);

    // Stringify the updated content and the frontmatter data
    let new_file_content = format!(
        "---\n{}---\n{}",
        serde_yaml::to_string(&data)?,
        updated_content
    );

    // Construct the full path of the file in the destination directory
    let output_file_path = Path::new(dest_dir).join(file.file_name().unwrap());

    // Write the new content into the file in the destination directory
    fs::write(output_file_path, new_file_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_copy_markdown_file() {
        // Set up a temporary directory and file for testing
        let temp_dir = tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("test.md");
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();

        // Sample content with frontmatter
        let content = r#"---
title: "Test Title"
description: "Test Description"
---
This is a test markdown file.
"#;

        temp_file.write_all(content.as_bytes()).unwrap();

        // Define the destination directory
        let dest_dir = tempdir().unwrap();

        // Call the function to copy the markdown file
        let result = copy_markdown_file(dest_dir.path().to_str().unwrap(), &temp_file_path);

        // Check if the operation was successful
        assert!(result.is_ok());

        // Check if the file was created in the destination directory
        let copied_file_path = dest_dir.path().join("test.md");
        assert!(copied_file_path.exists());

        // Verify the content of the copied file
        let copied_content = fs::read_to_string(copied_file_path).unwrap();
        assert!(copied_content.contains("# Test Title"));
        assert!(copied_content.contains("This is a test markdown file."));
    }
}
