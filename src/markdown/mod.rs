use std::{fs, path::PathBuf};

use copy::copy_markdown_file;

pub mod copy;

/// This function copies Markdown files from the source directory to the destination directory.
///
/// # Arguments
///
/// * `src_dir` - The source directory path.
/// * `dest_dir` - The destination directory path.
///
/// # Examples
///
/// ```
/// use zenn2press::copy_markdown_files;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let src_dir = "demo/zenn/articles";
///     let dest_dir = "demo/press/docs/articles";
///
///     copy_markdown_files(src_dir, dest_dir)?;
///     Ok(())
/// }
/// ```
pub fn copy_markdown_files(
    src_dir: &str,
    dest_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Copy Markdown:");
    println!("Starting to copy Markdown files...");

    // Read all files in the source directory
    let files: Vec<PathBuf> = fs::read_dir(src_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    for file in files {
        // Copy each Markdown file from the source directory to the dest directory.
        copy_markdown_file(dest_dir, &file)?;
    }

    println!("Copying of Markdown files is complete.");
    Ok(())
}
