use anyhow::{Error, Ok, Result};
use clap::Args;
use log::debug;
use nameof::name_of;
use rand::prelude::SliceRandom;

use crate::{
    algorithm::Algorithm,
    extensions::vec_extensions::VecExtensions,
    repetition_algorithm::{
        RepetitionAlgorithm, RepetitionAlgorithmInput, RepetitionAlgorithmResult,
    },
};

#[derive(Debug)]
pub struct Q1Result {
    _hit_percent: f32,
    _average: f32,
}

impl RepetitionAlgorithmResult<Vec<usize>, usize> for Q1Result {
    fn new(input: &Vec<usize>, series: Vec<usize>) -> Result<Self> {
        let max = input.iter().max().ok_or(Error::msg("Failed to get max"))?;

        let hit_count = series.iter().filter(|result| **result == *max).count();
        let hit_percent = (hit_count as f32 / series.len() as f32) * 100.0;
        let average = series.iter().sum::<usize>() as f32 / series.len() as f32;

        Ok(Q1Result {
            _hit_percent: hit_percent,
            _average: average,
        })
    }
}

pub struct SecreteryProblemAlgorithm();

impl Algorithm<Vec<usize>, usize> for SecreteryProblemAlgorithm {
    fn name() -> String {
        "Secretery problem".into()
    }

    fn run_internal(input: &Vec<usize>) -> Result<usize> {
        debug!("run_internal started");

        let n = input.len();
        let mut permutation = input.clone();
        permutation.shuffle(&mut rand::thread_rng());

        let threshold = permutation
            .iter()
            .take(n / 2)
            .max()
            .ok_or(Error::msg("failed to get maximum"))?;

        let result = permutation
            .iter()
            .skip(input.len() / 2)
            .filter(|item| **item >= *threshold)
            .next()
            .map_or(
                input.last().ok_or(Error::msg("no items in array"))?.clone(),
                |item| item.clone(),
            );

        debug!("run_internal finished [{}={}]", name_of!(result), result);

        Ok(result)
    }
}

type Q1Algorithm = RepetitionAlgorithm<SecreteryProblemAlgorithm, Q1Result, Vec<usize>, usize>;

#[derive(Debug, Args)]
pub struct Q1Command {
    ///Length of the array that will be generated
    #[clap(short, long, default_value = "1000")]
    array_length: usize,
    ///Amount of times the algorithm will run
    #[clap(short, long, default_value = "1000")]
    repeat_count: usize,
}

impl Q1Command {
    pub fn invoke(&self) -> Result<()> {
        Q1Algorithm::run(RepetitionAlgorithmInput {
            input: Vec::with_random_items_in_range(self.array_length, || 0..10000),
            repetition_count: self.repeat_count,
        })
    }
}
