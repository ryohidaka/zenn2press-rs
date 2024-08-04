use colored::Colorize;
use figlet_rs::FIGfont;

use crate::constants::DESCRIPTION;

pub fn print_figlet() {
    // Load the standard font for the FIGlet text representation
    let standard_font = FIGfont::standard().unwrap();

    // Convert the string "zenn2press" into a FIGlet text figure
    let figure = standard_font.convert("zenn2press");

    // Convert the FIGlet figure to a string, then apply blue and bold formatting
    let colored_figure = figure.unwrap().to_string().bright_blue().bold();

    // Print the colored FIGlet text
    println!("{}", colored_figure);

    // Print the description
    println!("== {} ==\n", DESCRIPTION.bold())
}
