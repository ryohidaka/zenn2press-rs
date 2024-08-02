use args::Args;
use clap::Parser;
use colored::Colorize;

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
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub config_file: Option<String>,
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

    // Print configuration details for debugging
    print_configuration_details(&args);

    // Return the populated `Properties` struct
    Properties {
        config_file: args.config_file,
        src_articles_dir: format!("{}{}", args.src_dir, ARTICLES_DIR),
        dest_articles_dir: args.dest_dir.clone(),
        src_images_dir: format!("{}{}", args.src_dir, IMAGES_DIR),
        dest_images_dir: args.dest_images_dir,
        include: Some(args.include),
        exclude: Some(args.exclude),
    }
}

/// Prints the configuration details, including the configuration file,
/// articles and images directories, and included/excluded files.
fn print_configuration_details(args: &Args) {
    println!(
        "{} {} {}",
        "[1/3]".bright_black().bold(),
        "📝",
        "Configuration Details:".bold().underline()
    );

    // Print config file
    print_config_file(&args.config_file);

    // Print articles and images directories
    print_directories(&args.src_dir, &args.dest_dir);

    // Print include and exclude files
    print_include_exclude_files(&args.include, &args.exclude);
}

/// Prints the configuration file path or "None" if not available.
fn print_config_file(config_file: &Option<String>) {
    match config_file {
        Some(config) => println!("- {} {}", "Configuration File:".green().bold(), config),
        None => println!(
            "- {} {}",
            "Configuration File:".green().bold(),
            "None".yellow().bold()
        ),
    }
}

/// Prints the source and destination directories for articles and images.
fn print_directories(src_dir: &str, dest_dir: &str) {
    let src_articles_dir = format!("{}{}", src_dir, ARTICLES_DIR);
    let src_images_dir = format!("{}{}", src_dir, IMAGES_DIR);

    println!(
        "- {} {:?} {} {:?}",
        "Articles:".green().bold(),
        src_articles_dir,
        "=>".yellow().bold(),
        dest_dir
    );
    println!(
        "- {} {:?} {} {:?}",
        "Images:".green().bold(),
        src_images_dir,
        "=>".yellow().bold(),
        dest_dir
    );
}

/// Prints the list of included and excluded files, or "None" if the lists are empty.
fn print_include_exclude_files(include: &Vec<String>, exclude: &Vec<String>) {
    // Include Files handling
    println!("- {}", "Include Files:".green().bold());
    if include.is_empty() {
        println!("\t- {}", "None");
    } else {
        for file in include {
            println!("\t- {}", file);
        }
    }

    // Exclude Files handling
    println!("- {}", "Exclude Files:".green().bold());
    if exclude.is_empty() {
        println!("\t- {}", "None");
    } else {
        for file in exclude {
            println!("\t- {}", file);
        }
    }
}
