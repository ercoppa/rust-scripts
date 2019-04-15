// cargo-deps: colored="*"

extern crate colored;

use colored::*;

fn main() {
    println!("{}", "this is not blue".green().bold().on_black());
    println!("{}", "Error".red().bold().on_black());
}
