use crate::recover_calibration_data;
use rand::prelude::*;
use crate::NUM_STR_PAIRS;


const LOW_CASE_ALPHANUMERIC_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

struct LowCaseAlphanumeric;

impl Distribution<char> for LowCaseAlphanumeric {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        LOW_CASE_ALPHANUMERIC_CHARSET.chars().choose(rng).unwrap()
    }
}


fn gen_rand_str_with_len(rng: &mut StdRng, str_len: usize) -> String {
    rng
            .sample_iter(&LowCaseAlphanumeric)
            .take(str_len)
            .map(char::from)
            .collect()
}

#[derive(Debug)]
struct TestData {
    line: String,  // line with coded calibration_value
    calibration_value: u128,  
}

impl TestData {
    fn gen_with_rng(
        rng: &mut StdRng,
    ) -> Self {
        let (start_num_str, start_num) = *NUM_STR_PAIRS
            .choose(rng)
            .expect("cannot choose elem from NUM_STR_PAIRS");
        let (end_num_str, end_num) = *NUM_STR_PAIRS
            .choose(rng)
            .expect("cannot choose elem from NUM_STR_PAIRS");

        let pre_str_len = rng.gen_range(5..10);
        let mut pre_str: String = gen_rand_str_with_len(rng, pre_str_len);

        let post_str_len = rng.gen_range(5..10);
        let mut post_str: String = gen_rand_str_with_len(rng, post_str_len);

        let middle_str_len = rng.gen_range(10..25);
        let middle_str = gen_rand_str_with_len(rng, middle_str_len);

        // pre_str and post_str may be accedentially generated with number substing
        for num_str in NUM_STR_PAIRS.iter().map(|(num_str, _)| num_str) {
            pre_str = pre_str.replace(num_str, "");
            post_str = post_str.replace(num_str, "");
        }

        Self {
            line: format!("{}{}{}{}{}", pre_str, start_num_str, middle_str, end_num_str, post_str),
            calibration_value: (start_num as u128 * 10) + (end_num as u128),
        }
    }
}

#[derive(Clone)]
struct TestDataGenerator {
    rng: StdRng,
}

impl TestDataGenerator {
    fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed)
        }
    }
}

impl Iterator for TestDataGenerator {
    type Item = TestData;

    fn next(&mut self) -> Option<Self::Item> {
        // let line = "oneabctwo".to_string();
        // let calibration_value = 12;
        let test_data = TestData::gen_with_rng(&mut self.rng);

        Some(test_data)
    }
}

#[test]
fn test_recover_calibration_data() {
    let test_seed = 123;
    let test_data_iter = TestDataGenerator::new(test_seed)
        .take(10);

    let lines_iter = test_data_iter.clone().map(|test_data| test_data.line);
    let sum = recover_calibration_data(lines_iter);

    let expected_sum = test_data_iter.fold(
        0,
        |acc, x| acc + x.calibration_value,
    );

    assert_eq!(sum, expected_sum);
}