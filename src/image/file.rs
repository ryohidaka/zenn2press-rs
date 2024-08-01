use futures::future::BoxFuture;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Recursively retrieves all file paths in a directory.
///
/// This function traverses a directory and returns a vector of file paths.
///
/// # Arguments
///
/// * `dir_path` - The path of the directory to search.
///
/// # Returns
///
/// * `BoxFuture<'a, io::Result<Vec<PathBuf>>>` - A future that resolves to a vector of file paths.
pub fn get_file_paths<'a>(dir_path: &'a str) -> BoxFuture<'a, io::Result<Vec<PathBuf>>> {
    Box::pin(async move {
        let mut entries: Vec<PathBuf> = Vec::new();
        let dir = fs::read_dir(dir_path)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            // If the entry is a directory, recursively get file paths
            if path.is_dir() {
                let sub_entries = get_file_paths(path.to_str().unwrap()).await?;
                entries.extend(sub_entries);
            } else {
                entries.push(path);
            }
        }

        Ok(entries)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};

    #[tokio::test]
    async fn test_get_file_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Create test files and directories
        let sub_dir = temp_dir_path.join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let file1 = temp_dir_path.join("file1.jpg");
        let file2 = sub_dir.join("file2.png");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let file_paths = get_file_paths(temp_dir_path.to_str().unwrap())
            .await
            .unwrap();
        assert_eq!(file_paths.len(), 2);
        assert!(file_paths.contains(&file1));
        assert!(file_paths.contains(&file2));
    }
}
