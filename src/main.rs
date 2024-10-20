use std::borrow::Borrow;

const test_data2: &str = "\
1abc2
";

fn main() {
    let sum = recover_calibration_data(test_data2.lines());
    println!("{}", sum);
}

/// extracts calibration value for each line and computes thier sum
fn recover_calibration_data<I, S>(lines_iter: I) -> u128 
where 
    I: Iterator<Item=S>,
    S: Borrow<str>,
    {
    let mut sum: u128 = 0;
    for line in lines_iter {
        let line = line.borrow();
        println!("{:?}", line);

        let mut left_index = 0;

        // go from left to find left-digit
        for i in 0..line.len() {
            let left_window = &line[i..];
            if let Some(left_digit) = slice_to_decimal_digit(left_window) {
                println!("{:?}", left_digit);
                sum += left_digit as u128 * 10;
                left_index = i;
                break;
            }
        }

        // go from right to find right-digit
        for i in (left_index..line.len()).rev() {
            let right_window = &line[i..];
            println!("{:?}", right_window);
            if let Some(right_digit) = slice_to_decimal_digit(right_window) {
                println!("{:?}", right_digit);
                sum += right_digit as u128;
                break;
            }
        }
    }

    sum
}

#[inline]
fn slice_to_decimal_digit(s: &str) -> Option<u32> {
    match s.as_bytes() {
        [b'z' | b'Z', b'e' | b'E', b'r' | b'R', b'o' | b'O', ..]                | [b'0', ..] => Some(0),
        [b'o' | b'O', b'n' | b'N', b'e' | b'E', ..]                             | [b'1', ..] => Some(1),
        [b't' | b'T', b'w' | b'W', b'o' | b'O', ..]                             | [b'2', ..] => Some(2),
        [b't' | b'T', b'h' | b'H', b'r' | b'R', b'e' | b'E', b'e' | b'E', ..]   | [b'3', ..] => Some(3),
        [b'f' | b'F', b'o' | b'O', b'u' | b'U', b'r' | b'R', ..]                | [b'4', ..] => Some(4),
        [b'f' | b'F', b'i' | b'I', b'v' | b'V', b'e' | b'E', ..]                | [b'5', ..] => Some(5),
        [b's' | b'S', b'i' | b'I', b'x' | b'X', ..]                             | [b'6', ..] => Some(6),
        [b's' | b'S', b'e' | b'E', b'v' | b'V', b'e' | b'E', b'n' | b'N', ..]   | [b'7', ..] => Some(7),
        [b'e' | b'E', b'i' | b'I', b'g' | b'G', b'h' | b'H', b't' | b'T', ..]   | [b'8', ..] => Some(8),
        [b'n' | b'N', b'i' | b'I', b'n' | b'N', b'e' | b'E', ..]                | [b'9', ..] => Some(9),
        _ => None,
    }
}


const NUM_STR_PAIRS: &[(&str, u8)] = &[
    ("zero", 0), ("0", 0),    
    ("one", 1), ("1", 1),
    ("two", 2), ("2", 2),
    ("three", 3), ("3", 3),
    ("four", 4), ("4", 4),
    ("five", 5), ("5", 5),
    ("six", 6), ("6", 6),
    ("seven", 7), ("7", 7),
    ("eight", 8), ("8", 8),
    ("nine", 9), ("9", 9),
];


// #[cfg(test)]
// mod tests {

// }

 #[cfg(test)]
 mod tests;