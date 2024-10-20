use utils::*;

mod utils;

#[cfg(test)]
mod tests;

const TEST_DATA: &str = "\
1abc2
zonabconesiix2neine!
";

fn main() {
    let sum = recover_calibration_data(TEST_DATA.lines());
    println!("result: {}", sum);
}
