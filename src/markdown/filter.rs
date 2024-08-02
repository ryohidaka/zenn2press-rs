use std::path::PathBuf;

/// Filters a list of files to include only markdown files, with optional include/exclude filters.
///
/// This function filters a list of `PathBuf` entries to include only those with a `.md` extension.
/// Additionally, it can apply include and exclude filters to narrow down the selection of files.
///
/// # Arguments
///
/// * `files` - A vector of `PathBuf` representing file paths.
/// * `include` - An optional vector of file names (without extensions) to explicitly include.
/// * `exclude` - An optional vector of file names (without extensions) to explicitly exclude.
///
/// # Returns
///
/// * `Vec<PathBuf>` - A vector of `PathBuf` entries representing the filtered list of markdown files.
pub fn filter_markdown_files(
    files: Vec<PathBuf>,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
) -> Vec<PathBuf> {
    // Filter the files to include only those with a .md extension
    let mut markdown_files: Vec<PathBuf> = files
        .into_iter()
        .filter(|file| file.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();

    // Apply the include filter if specified
    if let Some(include) = include {
        if !include.is_empty() {
            let include_with_md: Vec<String> = add_md_ext(include);
            markdown_files.retain(|file| {
                include_with_md.contains(&file.file_name().unwrap().to_string_lossy().to_string())
            });
        }
    }

    // Apply the exclude filter if specified
    if let Some(exclude) = exclude {
        if !exclude.is_empty() {
            let exclude_with_md: Vec<String> = add_md_ext(exclude);
            markdown_files.retain(|file| {
                !exclude_with_md.contains(&file.file_name().unwrap().to_string_lossy().to_string())
            });
        }
    }

    markdown_files
}

/// Adds the `.md` extension to a list of file names if it is not already present.
///
/// This utility function takes a list of file names (without extensions) and ensures they all
/// have a `.md` extension.
///
/// # Arguments
///
/// * `paths` - A vector of file name strings (without extensions).
///
/// # Returns
///
/// * `Vec<String>` - A vector of strings representing the file names with `.md` extensions.
fn add_md_ext(paths: Vec<&str>) -> Vec<String> {
    paths
        .into_iter()
        .map(|path| {
            if path.ends_with(".md") {
                path.to_string()
            } else {
                format!("{}.md", path)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_markdown_files() {
        let files = vec![
            PathBuf::from("file1.md"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.md"),
            PathBuf::from("file4.md"),
        ];

        // Test case: no include or exclude filters
        let result = filter_markdown_files(files.clone(), None, None);
        assert_eq!(
            result,
            vec![
                PathBuf::from("file1.md"),
                PathBuf::from("file3.md"),
                PathBuf::from("file4.md")
            ]
        );

        // Test case: include filter
        let include = Some(vec!["file1", "file3"]);
        let result = filter_markdown_files(files.clone(), include, None);
        assert_eq!(
            result,
            vec![PathBuf::from("file1.md"), PathBuf::from("file3.md")]
        );

        // Test case: exclude filter
        let exclude = Some(vec!["file3"]);
        let result = filter_markdown_files(files.clone(), None, exclude);
        assert_eq!(
            result,
            vec![PathBuf::from("file1.md"), PathBuf::from("file4.md")]
        );

        // Test case: both include and exclude filters
        let include = Some(vec!["file1", "file4"]);
        let exclude = Some(vec!["file4"]);
        let result = filter_markdown_files(files, include, exclude);
        assert_eq!(result, vec![PathBuf::from("file1.md")]);
    }

    #[test]
    fn test_add_md_ext() {
        // Test case: file names without extension
        let paths = vec!["file1", "file2", "file3"];
        let result = add_md_ext(paths);
        assert_eq!(
            result,
            vec![
                "file1.md".to_string(),
                "file2.md".to_string(),
                "file3.md".to_string()
            ]
        );

        // Test case: file names with and without extension
        let paths = vec!["file1.md", "file2", "file3.md"];
        let result = add_md_ext(paths);
        assert_eq!(
            result,
            vec![
                "file1.md".to_string(),
                "file2.md".to_string(),
                "file3.md".to_string()
            ]
        );
    }
}
