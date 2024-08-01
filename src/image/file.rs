use futures::future::BoxFuture;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Recursively retrieves all file paths in a directory, with optional include and exclude filters.
///
/// This function traverses a directory and returns a vector of file paths. It can optionally
/// include or exclude specific files based on substring matching.
///
/// # Arguments
///
/// * `dir_path` - The path of the directory to search.
/// * `include` - An optional vector of strings. Only files whose paths contain these substrings will be included.
/// * `exclude` - An optional vector of strings. Files whose paths contain these substrings will be excluded.
///
/// # Returns
///
/// * `BoxFuture<'a, io::Result<Vec<PathBuf>>>` - A future that resolves to a vector of file paths.
pub fn get_file_paths<'a>(
    dir_path: &'a str,
    include: Option<Vec<&'a str>>,
    exclude: Option<Vec<&'a str>>,
) -> BoxFuture<'a, io::Result<Vec<PathBuf>>> {
    Box::pin(async move {
        let mut entries: Vec<PathBuf> = Vec::new();
        let dir = fs::read_dir(dir_path)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_str().unwrap_or_default();

            // If include filter is provided, only include matching files
            if let Some(ref include) = include {
                if !include.is_empty() && !include.iter().any(|inc| path_str.contains(inc)) {
                    continue;
                }
            }

            // If exclude filter is provided, skip matching files
            if let Some(ref exclude) = exclude {
                if !exclude.is_empty() && exclude.iter().any(|exc| path_str.contains(exc)) {
                    continue;
                }
            }

            // If the entry is a directory, recursively get file paths
            if path.is_dir() {
                let sub_entries =
                    get_file_paths(path.to_str().unwrap(), include.clone(), exclude.clone())
                        .await?;
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

        // Test with no filters
        let file_paths = get_file_paths(temp_dir_path.to_str().unwrap(), None, None)
            .await
            .unwrap();
        assert_eq!(file_paths.len(), 2);
        assert!(file_paths.contains(&file1));
        assert!(file_paths.contains(&file2));

        // Test with include filter
        let include_filter = vec!["file1"];
        let file_paths =
            get_file_paths(temp_dir_path.to_str().unwrap(), Some(include_filter), None)
                .await
                .unwrap();
        assert_eq!(file_paths.len(), 1);
        assert!(file_paths.contains(&file1));
        assert!(!file_paths.contains(&file2));

        // Test with exclude filter
        let exclude_filter = vec!["file1"];
        let file_paths =
            get_file_paths(temp_dir_path.to_str().unwrap(), None, Some(exclude_filter))
                .await
                .unwrap();
        assert_eq!(file_paths.len(), 1);
        assert!(!file_paths.contains(&file1));
        assert!(file_paths.contains(&file2));
    }
}
