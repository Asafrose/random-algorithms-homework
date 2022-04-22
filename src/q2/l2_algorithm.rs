use anyhow::Result;
use log::debug;
use nameof::name_of;

use crate::{common::algorithm::Algorithm, q2::hash_function::HashFunction};

pub struct L2Algorithm{
    array: Vec<i32>
}

impl Algorithm<Vec<i32>, i32> for L2Algorithm {
    fn name() -> String {
        "l2 algorithm".into()
    }

    fn run_internal(&self) -> Result<i32> {
        debug!("run_internal started");

        let hash_function = HashFunction::new(0..self.array.len(), [-1, 1]);

        let result = (0..self.array.len())
            .map(|num| hash_function.get_value(&num) * self.array[num])
            .sum::<i32>()
            .pow(2);

        debug!("run_internal finished [{}={}]", name_of!(result), result);

        Ok(result)
    }

    fn new(input: Vec<i32>, _is_update_progress: bool) -> Self {
        Self{
            array: input
        }
    }
}
