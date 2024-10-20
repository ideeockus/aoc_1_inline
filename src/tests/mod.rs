use crate::utils::extract_calibration_value;
use crate::utils::recover_calibration_data;
use crate::utils::NUM_STR_PAIRS;
use rand::prelude::*;

// charset to generate random lowercase ascii strings
const LOW_CASE_ALPHANUMERIC_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

struct LowCaseAlphanumeric;

impl Distribution<char> for LowCaseAlphanumeric {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        LOW_CASE_ALPHANUMERIC_CHARSET.chars().choose(rng).unwrap()
    }
}

// using this charset, it is impossible to accidently generate a number
const LOW_CASE_NUMERIC_SAFE_CHARSET: &str = "abcdjklmpqy";

struct LowCaseNoNumeric;

impl Distribution<char> for LowCaseNoNumeric {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        LOW_CASE_NUMERIC_SAFE_CHARSET.chars().choose(rng).unwrap()
    }
}

fn gen_rand_str_with_len<D>(rng: &mut StdRng, str_len: usize, distrib: D) -> String
where
    D: Distribution<char>,
{
    rng.sample_iter(distrib).take(str_len).map(char::from).collect()
}

#[derive(Debug)]
struct TestData {
    line: String, // line with coded calibration_value
    calibration_value: u128,
}

impl TestData {
    /// generate calibration value +  test line in format:
    /// <first part + first digit + middle part (possibly with digits) + second digit + last part>
    fn gen_with_rng(rng: &mut StdRng) -> Self {
        let (start_num_str, start_num) = *NUM_STR_PAIRS
            .choose(rng)
            .expect("cannot choose elem from NUM_STR_PAIRS");
        let (end_num_str, end_num) = *NUM_STR_PAIRS
            .choose(rng)
            .expect("cannot choose elem from NUM_STR_PAIRS");

        let pre_str_len = rng.gen_range(5..10);
        let pre_str: String = gen_rand_str_with_len(rng, pre_str_len, LowCaseNoNumeric);

        let post_str_len = rng.gen_range(5..10);
        let post_str: String = gen_rand_str_with_len(rng, post_str_len, LowCaseNoNumeric);

        let middle_str_len = rng.gen_range(10..25);
        let middle_str = gen_rand_str_with_len(rng, middle_str_len, LowCaseAlphanumeric);

        Self {
            line: format!("{}{}{}{}{}", pre_str, start_num_str, middle_str, end_num_str, post_str),
            calibration_value: (start_num as u128 * 10) + (end_num as u128),
        }
    }
}

#[derive(Clone)]
struct TestDataGenerator {
    rng: StdRng,
    randomize_case: bool,
}

impl TestDataGenerator {
    fn new(seed: u64, randomize_case: bool) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            randomize_case,
        }
    }
}

impl Iterator for TestDataGenerator {
    type Item = TestData;

    fn next(&mut self) -> Option<Self::Item> {
        let mut test_data = TestData::gen_with_rng(&mut self.rng);

        if self.randomize_case {
            test_data.line = test_data
                .line
                .chars()
                .map(|c| {
                    if self.rng.gen() {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect();
        }

        Some(test_data)
    }
}

#[test]
fn test_extract_calibration_value() {
    let test_seed = 123;
    let test_data_iter = TestDataGenerator::new(test_seed, false).take(1_000);

    for test_data in test_data_iter {
        let calibration_value = extract_calibration_value(test_data.line.as_str());
        let expected_calibration_value = test_data.calibration_value;
        println!(
            "line {} | extracted: {}, expected: {}",
            test_data.line, calibration_value, expected_calibration_value,
        );
        assert_eq!(calibration_value, expected_calibration_value)
    }
}

#[test]
fn test_recover_calibration_data_lowcase() {
    let test_seed = 123;
    let test_data_iter = TestDataGenerator::new(test_seed, false).take(10_000);

    let lines_iter = test_data_iter.clone().map(|test_data| test_data.line);
    let sum = recover_calibration_data(lines_iter);

    let expected_sum = test_data_iter.fold(0, |acc, x| acc + x.calibration_value);

    assert_eq!(sum, expected_sum);
}

#[test]
fn test_recover_calibration_data_different_case() {
    let test_seed = 456;
    let test_data_iter = TestDataGenerator::new(test_seed, true).take(20_000);

    let lines_iter = test_data_iter.clone().map(|test_data| test_data.line);
    let sum = recover_calibration_data(lines_iter);

    let expected_sum = test_data_iter.fold(0, |acc, x| acc + x.calibration_value);

    assert_eq!(sum, expected_sum);
}
