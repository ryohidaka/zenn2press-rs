use std::{fs, path::PathBuf};

use config::read_config_file;
use copy::copy_markdown_file;
use filter::filter_markdown_files;

use crate::progress_bar::get_pb;

pub mod config;
pub mod copy;
pub mod filter;
pub mod frontmatter;

/// This function copies Markdown files from the source directory to the destination directory,
/// with options for filtering and configuring frontmatter.
///
/// # Arguments
///
/// * `src_dir` - The source directory path.
/// * `dest_dir` - The destination directory path.
/// * `config_file` - An optional path to a configuration file for frontmatter.
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
///     let config_file = Some("demo/zenn2press-config.json");
///     let include = Some(vec!["sample-article-1"]);
///     let exclude = None;
///
///     copy_markdown_files(src_dir, dest_dir, config_file, include, exclude)?;
///     Ok(())
/// }
/// ```
pub fn copy_markdown_files(
    src_dir: &str,
    dest_dir: &str,
    config_file: Option<&str>,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read all files in the source directory
    let files: Vec<PathBuf> = fs::read_dir(src_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Filter the Markdown files based on the include and exclude lists
    let markdown_files = filter_markdown_files(files, include, exclude);

    // Read the configuration file for frontmatter, if provided
    let frontmatter_config = if let Some(config_file) = config_file {
        Some(read_config_file(config_file)?)
    } else {
        None
    };

    let file_length = markdown_files.len().try_into().unwrap();
    let pb = get_pb(file_length);

    for file in markdown_files {
        let filename = file.file_name().unwrap().to_string_lossy().to_string();
        pb.set_message(filename);

        // Copy each Markdown file from the source directory to the dest directory.
        copy_markdown_file(dest_dir, &file, frontmatter_config.as_ref())?;
        pb.inc(1)
    }
    pb.finish_with_message("Completed.");

    Ok(())
}
