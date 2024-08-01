use utils::property::get_properties;
use zenn2press::{image::copy_images, markdown::copy_markdown_files};

mod constants;

mod utils;

#[tokio::main]
async fn main() {
    // Fetch properties from configuration
    let properties = get_properties();

    // Convert Vec<String> to Vec<&str> for include and exclude lists
    let include: Option<Vec<&str>> = properties
        .include
        .as_ref()
        .map(|v| v.iter().map(|s| s.as_str()).collect());
    let exclude: Option<Vec<&str>> = properties
        .exclude
        .as_ref()
        .map(|v| v.iter().map(|s| s.as_str()).collect());

    // Copy markdown files from srcDir to destDir using properties
    copy_markdown_files(
        properties.src_articles_dir.as_str(),
        properties.dest_articles_dir.as_str(),
        include.clone(),
        exclude.clone(),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error copying markdown files: {}", e);
    });

    // Copy images from srcDir to destDir using properties
    match copy_images(
        properties.src_images_dir.as_str(),
        properties.dest_images_dir.as_str(),
        include.clone(),
        exclude.clone(),
    )
    .await
    {
        Ok(_) => println!("Images copied successfully."),
        Err(e) => eprintln!("Error copying images: {}", e),
    }
}
