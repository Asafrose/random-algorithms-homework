use anyhow::Result;

use crate::{
    algorithm::Algorithm,
    repetition_algorithm::{
        RepetitionAlgorithm, RepetitionAlgorithmInput, RepetitionAlgorithmResult,
    },
};

use super::l2_algorithm::L2Algorithm;

#[derive(Debug)]
pub struct Q2NativeAlgorithmResult {
    _average: f32,
    _l2_norm: i32,
}

impl RepetitionAlgorithmResult<Vec<i32>, i32> for Q2NativeAlgorithmResult {
    fn new(input: &Vec<i32>, series: Vec<i32>) -> Result<Self> {
        let average = series.iter().sum::<i32>() as f32 / series.len() as f32;
        let l2_norm = L2Algorithm::get_l2_norm(&input);

        Ok(Q2NativeAlgorithmResult {
            _average: average,
            _l2_norm: l2_norm,
        })
    }
}

pub struct Q2NaiveAlgorithm;

impl Algorithm<Vec<i32>, Q2NativeAlgorithmResult> for Q2NaiveAlgorithm {
    fn name() -> String {
        "q2 naive algorithm".into()
    }

    fn run_internal(input: &Vec<i32>) -> Result<Q2NativeAlgorithmResult> {
        RepetitionAlgorithm::<L2Algorithm, Q2NativeAlgorithmResult, Vec<i32>, i32>::run_internal(
            &RepetitionAlgorithmInput {
                input: input.clone(),
                repetition_count: 1000,
            },
        )
    }
}