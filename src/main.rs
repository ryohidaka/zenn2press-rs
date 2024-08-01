use utils::property::get_properties;
use zenn2press::markdown::copy_markdown_files;

mod constants;

mod utils;

fn main() {
    // Fetch properties from configuration
    let properties = get_properties();

    // Copy markdown files from srcDir to destDir using properties
    copy_markdown_files(
        properties.src_articles_dir.as_str(),
        properties.dest_articles_dir.as_str(),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error copying markdown files: {}", e);
    });
}
