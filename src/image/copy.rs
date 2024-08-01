use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Copies files to a target directory.
///
/// This function iterates over a list of file paths, determines their relative paths
/// with respect to a source directory, and copies them to a destination directory while
/// maintaining the directory structure.
///
/// # Arguments
///
/// * `file_paths` - A slice of `PathBuf` containing the paths of files to copy.
/// * `src_dir` - The source directory as a string slice.
/// * `dest_dir` - The destination directory as a string slice.
///
/// # Returns
///
/// * `io::Result<()>` - Returns `Ok(())` on success or an `io::Error` on failure.
pub fn copy_files(file_paths: &[PathBuf], src_dir: &str, dest_dir: &str) -> io::Result<()> {
    println!("Copying files...");

    for file in file_paths {
        // Determine the file's relative path from the source directory
        let relative_path = file.strip_prefix(src_dir).unwrap();
        let target_path = Path::new(dest_dir).join(relative_path);

        // Create the necessary directory structure in the destination
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy the file to the destination
        fs::copy(file, target_path)?;
    }

    println!("Copying of files is complete.");
    Ok(())
}
