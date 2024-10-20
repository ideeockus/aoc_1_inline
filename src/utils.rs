use std::borrow::Borrow;

/// computes sum of extracted calibration values
pub fn recover_calibration_data<I, S>(lines_iter: I) -> u128
where
    I: Iterator<Item = S>,
    S: Borrow<str>,
{
    let mut sum: u128 = 0;
    for line in lines_iter {
        sum += extract_calibration_value(line)
    }

    sum
}

/// extracts calibration value from line
pub fn extract_calibration_value<S>(line: S) -> u128
where
    S: Borrow<str>,
{
    let line: &str = line.borrow();
    let mut left_index = 0;
    let mut calibration_value = 0;

    // go from left to find left-digit
    for i in 0..line.len() {
        let left_window = &line[i..];
        if let Some(left_digit) = slice_to_decimal_digit(left_window) {
            calibration_value += left_digit as u128 * 10;
            left_index = i;
            break;
        }
    }

    // go from right to find right-digit
    for i in (left_index..line.len()).rev() {
        let right_window = &line[i..];
        if let Some(right_digit) = slice_to_decimal_digit(right_window) {
            calibration_value += right_digit as u128;
            break;
        }
    }

    calibration_value
}

#[rustfmt::skip]
#[inline]
pub fn slice_to_decimal_digit(s: &str) -> Option<u32> {
    // в целом тут можно было через str::eq_ignore_ascii_case, но так кажется эффективнее + меньше ветвлений
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
