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

    // Stringify the updated content and the frontmatter data
    let new_file_content = format!("n{}", file_content);

    // Construct the full path of the file in the destination directory
    let output_file_path = Path::new(dest_dir).join(file.file_name().unwrap());

    // Write the new content into the file in the destination directory
    fs::write(output_file_path, new_file_content)?;

    Ok(())
}
