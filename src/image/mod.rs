use std::io;

use copy::copy_files;
use file::get_file_paths;

pub mod copy;
pub mod file;

/// This function copies all files and directories from the source directory to the destination directory.
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
/// use zenn2press::copy_images;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let src_dir = "demo/zenn/images";
///     let dest_dir = "demo/press/docs/public/images";
///     let include = Some(vec!["sample-article-1"]);
///     let exclude = None;
///
///     copy_images(src_dir, dest_dir, include, exclude).await?;
///     Ok(())
/// }
/// ```
pub async fn copy_images(
    src_dir: &str,
    dest_dir: &str,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
) -> io::Result<()> {
    // Get all file paths from the source directory
    let file_paths = get_file_paths(src_dir, include, exclude).await?;
    // Copy each file to the destination directory
    copy_files(&file_paths, src_dir, dest_dir)
}
