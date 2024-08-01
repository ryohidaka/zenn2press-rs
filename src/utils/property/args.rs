use crate::constants::DESCRIPTION;
use clap::Parser;

/// `Args` is a structure representing the command-line arguments accepted by the application.
/// The `clap` crate is used to parse and validate these arguments.
#[derive(Parser)]
#[command(version, about = DESCRIPTION)]
#[command(next_line_help = true)]
pub struct Args {
    /// Path of the root directory of Zenn content.
    #[arg(short, long, value_name = "DIR")]
    pub src_dir: String,

    /// The VitePress directory path (e.g. docs/entries) where you want to place the markdown for the articles.
    #[arg(short = 'd', long, value_name = "DIR")]
    pub dest_dir: String,

    /// The VitePress directory path (e.g. public) where the image will be placed.
    #[arg(short = 'm', long, value_name = "DIR")]
    pub dest_images_dir: String,

    /// File names to include.
    #[arg(short, long, value_name = "<FILE>", value_delimiter = ',')]
    pub include: Vec<String>,

    /// File names to exclude.
    #[arg(short, long, value_name = "<FILE>", value_delimiter = ',')]
    pub exclude: Vec<String>,
}
