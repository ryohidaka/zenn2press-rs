use utils::property::get_properties;
use zenn2press::{image::copy_images, markdown::copy_markdown_files};

mod constants;

mod utils;

#[tokio::main]
async fn main() {
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

    // Copy images from srcDir to destDir using properties
    match copy_images(
        properties.src_images_dir.as_str(),
        properties.dest_images_dir.as_str(),
    )
    .await
    {
        Ok(_) => println!("Images copied successfully."),
        Err(e) => eprintln!("Error copying images: {}", e),
    }
}
