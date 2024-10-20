use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use utils::*;

mod utils;

#[cfg(test)]
mod tests;

fn main() {
    println!("{:?}", env::args().collect::<Vec<String>>());
    let mut cli_args = env::args();
    let _command = cli_args.next();
    let filename = match cli_args.next() {
        Some(v) => v,
        None => {
            eprintln!("No filename provided");
            process::exit(1);
        }
    };

    let file = File::open(filename).expect("Cannot open provided file");
    let reader = BufReader::new(file);

    let sum = recover_calibration_data(reader.lines().map(|line| line.unwrap()));
    println!("result: {}", sum);
}
