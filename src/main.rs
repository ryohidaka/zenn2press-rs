use std::time::Instant;

use colored::Colorize;
use indicatif::HumanDuration;

use utils::{figlet::print_figlet, property::get_properties};
use zenn2press::{image::copy_images, markdown::copy_markdown_files};

mod constants;
mod utils;

#[tokio::main]
async fn main() {
    // Record the start time of the process
    let started = Instant::now();

    // Print Figlet text
    print_figlet();

    // Fetch properties from the configuration
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

    // Print status update for copying markdown files
    println!(
        "{} {} {}",
        "[2/3]".bright_black().bold(),
        "📚",
        "Copy markdown files:".bold().underline()
    );

    // Copy markdown files from srcDir to destDir using properties
    // If an error occurs, print the error message
    copy_markdown_files(
        properties.src_articles_dir.as_str(),
        properties.dest_articles_dir.as_str(),
        properties.config_file.as_deref(),
        include.clone(),
        exclude.clone(),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error copying markdown files: {}", e);
    });

    // Print status update for copying image files
    println!(
        "{} {} {}",
        "[3/3]".bright_black().bold(),
        "🖼️",
        "Copy image files:".bold().underline()
    );

    // Copy images from srcDir to destDir using properties
    // Await the result and handle success or error
    match copy_images(
        properties.src_images_dir.as_str(),
        properties.dest_images_dir.as_str(),
        include.clone(),
        exclude.clone(),
    )
    .await
    {
        Ok(_) => println!("\n\n{} Done in {}", "✨", HumanDuration(started.elapsed())),
        Err(e) => eprintln!("Error copying images: {}", e),
    }
}
