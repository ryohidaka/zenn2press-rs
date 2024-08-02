use indicatif::{ProgressBar, ProgressStyle};

/// Creates and returns a ProgressBar with a custom style.
///
/// # Arguments
///
/// * `len` - The total length or count of the progress bar.
///
/// # Returns
///
/// * `ProgressBar` - A new ProgressBar instance with the specified length and style.
pub fn get_pb(len: u64) -> ProgressBar {
    // Create a new ProgressBar with the specified length
    let pb = ProgressBar::new(len);

    // Define a custom progress bar style template
    let sty = ProgressStyle::with_template("{bar:40} {pos:>2}/{len:2} {msg}")
        .expect("Failed to create progress style template");

    // Set the style of the progress bar
    pb.set_style(sty.clone());

    pb
}
