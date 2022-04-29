use anyhow::Result;
use log::debug;
use nameof::name_of;

use crate::{common::algorithm::Algorithm, q2::hash_function::HashFunction};

pub struct L2Algorithm {
    array: Vec<f64>,
}

impl L2Algorithm {
    pub fn new(array: Vec<f64>) -> Self {
        Self { array }
    }
}

impl Algorithm for L2Algorithm {
    type Input = Vec<f64>;
    type Output = f64;

    fn name(&self) -> String {
        "l2 algorithm".into()
    }

    fn input(&self) -> Self::Input {
        self.array.clone()
    }

    fn run_internal<F: Fn() + Send + Sync>(&self, update_progress: F) -> Result<f64> {
        debug!("run_internal started");

        let hash_function = HashFunction::new(0..self.array.len(), [-1 as f64, 1 as f64]);

        let result = (0..self.array.len())
            .map(|num| hash_function.get_value(&num) * self.array[num])
            .sum::<f64>()
            .powi(2);

        debug!("run_internal finished [{}={}]", name_of!(result), result);

        update_progress();

        Ok(result)
    }
}
