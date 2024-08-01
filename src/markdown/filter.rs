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
