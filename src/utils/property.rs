use args::Args;
use clap::Parser;

use crate::constants::{ARTICLES_DIR, IMAGES_DIR};

mod args;

/// `Properties` is a structure that holds the configuration and paths
/// derived from the command-line arguments.
#[derive(Debug)]
pub struct Properties {
    pub src_articles_dir: String,
    pub dest_articles_dir: String,
    pub src_images_dir: String,
    pub dest_images_dir: String,
}

/// This function parses the command-line arguments and returns a `Properties` struct
/// containing the derived paths and configurations.
///
/// # Returns
///
/// * `Properties` - A structure holding all the relevant paths and options derived from the arguments.
pub fn get_properties() -> Properties {
    // Parse the command-line arguments into an `Args` struct
    let args = Args::parse();

    // Derive paths for the source articles and images
    let src_articles_dir = format!("{}{}", args.src_dir, ARTICLES_DIR);
    let src_images_dir = format!("{}{}", args.src_dir, IMAGES_DIR);

    // Clone the destination articles directory path
    let dest_articles_dir = args.dest_dir.clone();

    // Print configuration details for debugging
    println!(
        "Articles: {:?} => {:?}",
        src_articles_dir, dest_articles_dir
    );
    println!("Images: {:?} => {:?}", src_images_dir, args.dest_images_dir);

    // Return the populated `Properties` struct
    Properties {
        src_articles_dir,
        dest_articles_dir,
        src_images_dir,
        dest_images_dir: args.dest_images_dir,
    }
}
