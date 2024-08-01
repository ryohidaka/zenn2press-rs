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
