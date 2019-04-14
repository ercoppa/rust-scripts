use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <csv_file>", args[0]);
        process::exit(1);
    }

    let csv_file = &args[1];
    println!("Reading file: {}", csv_file);

    let f = File::open(csv_file)?;
    for line in BufReader::new(f).lines() {
        println!("{}", line?.replace(",", "\t"));
    }

    Ok(())
}
