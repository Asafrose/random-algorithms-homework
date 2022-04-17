use anyhow::Result;
use log::debug;
use nameof::name_of;

use crate::{algorithm::Algorithm, q2::hash_function::HashFunction};

pub struct L2Algorithm();

impl Algorithm<Vec<i32>, i32> for L2Algorithm {
    fn name() -> String {
        "l2 algorithm".into()
    }

    fn run_internal(input: &Vec<i32>) -> Result<i32> {
        debug!("run_internal started");

        let hash_function = HashFunction::new(0..input.len(), [-1, 1]);

        let result = (0..input.len())
            .map(|num| hash_function.get_value(&num) * input[num])
            .sum::<i32>()
            .pow(2);

        debug!("run_internal finished [{}={}]", name_of!(result), result);

        Ok(result)
    }
}

impl L2Algorithm {
    pub fn get_l2_norm(array: &Vec<i32>) -> i32 {
        array.iter().map(|num| num.pow(2)).sum()
    }
}
