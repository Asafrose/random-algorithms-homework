use anyhow::{Error, Ok, Result};
use clap::Args;
use log::debug;
use nameof::name_of;
use rand::prelude::SliceRandom;

use crate::{
    common::{algorithm::Algorithm, repeat::IntoRepeat, reduce::IntoReduce},
    extensions::vec_extensions::SampleUniformVecExtensions,
};

#[derive(Debug)]
pub struct Q1Result {
    _hit_percent: f32,
    _average: f32,
}

pub struct SecreteryProblemAlgorithm {
    array: Vec<usize>,
}

impl Algorithm for SecreteryProblemAlgorithm {
    type Input = Vec<usize>;
    type Output = usize;

    fn name(&self) -> String {
        "Secretery problem".into()
    }

    fn input(&self) -> Self::Input {
        self.array.clone()
    }

    fn run_internal<F: Fn() + Send + Sync>(&self, update_progress: F) -> Result<usize> {
        debug!("run_internal started");

        let n = self.array.len();
        let mut permutation = self.array.clone();
        permutation.shuffle(&mut rand::thread_rng());

        let threshold = permutation
            .iter()
            .take(n / 2)
            .max()
            .ok_or(Error::msg("failed to get maximum"))?;

        let result = permutation
            .iter()
            .skip(self.array.len() / 2)
            .filter(|item| **item >= *threshold)
            .next()
            .map_or(
                self.array
                    .last()
                    .ok_or(Error::msg("no items in array"))?
                    .clone(),
                |item| item.clone(),
            );

        debug!("run_internal finished [{}={}]", name_of!(result), result);

        update_progress();

        Ok(result)
    }
}

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
        let array = Vec::with_random_items_in_range(self.array_length, || 0..10000);

        SecreteryProblemAlgorithm { array: array.clone() }
            .repeat(self.repeat_count)
            .reduce(move |series| {
                let max = array.iter().max().ok_or(Error::msg("Failed to get max"))?;

                let hit_count = series.iter().filter(|result| **result == *max).count();
                let hit_percent = (hit_count as f32 / series.len() as f32) * 100.0;
                let average = series.iter().sum::<usize>() as f32 / series.len() as f32;

                Ok(Q1Result {
                    _hit_percent: hit_percent,
                    _average: average,
                })
            })
            .run()
    }
}
