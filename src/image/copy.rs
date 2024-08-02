use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::progress_bar::get_pb;

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
    let file_length = file_paths.len().try_into().unwrap();

    let pb = get_pb(file_length);

    for file in file_paths {
        let filename = file.file_name().unwrap().to_string_lossy().to_string();
        pb.set_message(filename);

        // Determine the file's relative path from the source directory
        let relative_path = file.strip_prefix(src_dir).unwrap();
        let target_path = Path::new(dest_dir).join(relative_path);

        // Create the necessary directory structure in the destination
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy the file to the destination
        fs::copy(file, target_path)?;

        pb.inc(1)
    }

    pb.finish_with_message("Completed.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_copy_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let src_dir = temp_dir_path.join("src");
        let dest_dir = temp_dir_path.join("dest");

        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();

        // Create test files
        let file1 = src_dir.join("file1.txt");
        let file2 = src_dir.join("file2.txt");
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Test file 1").unwrap();
        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "Test file 2").unwrap();

        // Test copying files
        let file_paths = vec![file1.clone(), file2.clone()];
        copy_files(
            &file_paths,
            src_dir.to_str().unwrap(),
            dest_dir.to_str().unwrap(),
        )
        .unwrap();

        let copied_file1 = dest_dir.join("file1.txt");
        let copied_file2 = dest_dir.join("file2.txt");

        assert!(copied_file1.exists());
        assert!(copied_file2.exists());

        // Verify contents of the copied files
        let contents1 = fs::read_to_string(copied_file1).unwrap();
        let contents2 = fs::read_to_string(copied_file2).unwrap();

        assert_eq!(contents1.trim(), "Test file 1");
        assert_eq!(contents2.trim(), "Test file 2");
    }
}
