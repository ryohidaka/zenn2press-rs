use std::{fs, path::PathBuf};

use copy::copy_markdown_file;
use filter::filter_markdown_files;

pub mod copy;
pub mod filter;
pub mod frontmatter;

/// This function copies Markdown files from the source directory to the destination directory,
/// with options for filtering.
///
/// # Arguments
///
/// * `src_dir` - The source directory path.
/// * `dest_dir` - The destination directory path.
/// * `include` - The optional list of files to include in the processing.
/// * `exclude` - The optional list of files to exclude from the processing.
///
/// # Examples
///
/// ```
/// use zenn2press::copy_markdown_files;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let src_dir = "demo/zenn/articles";
///     let dest_dir = "demo/press/docs/articles";
///     let include = Some(vec!["sample-article-1"]);
///     let exclude = None;
///
///     copy_markdown_files(src_dir, dest_dir, include, exclude)?;
///     Ok(())
/// }
/// ```
pub fn copy_markdown_files(
    src_dir: &str,
    dest_dir: &str,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Copy Markdown:");
    println!("Starting to copy Markdown files...");

    // Read all files in the source directory
    let files: Vec<PathBuf> = fs::read_dir(src_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Filter the Markdown files based on the include and exclude lists
    let markdown_files = filter_markdown_files(files, include, exclude);

    for file in markdown_files {
        // Copy each Markdown file from the source directory to the dest directory.
        copy_markdown_file(dest_dir, &file)?;
    }

    println!("Copying of Markdown files is complete.");
    Ok(())
}
