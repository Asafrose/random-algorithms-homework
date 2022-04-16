use anyhow::{Error, Ok, Result};
use clap::Args;
use log::{debug, info};
use nameof::name_of;
use rand::{prelude::SliceRandom};

use crate::extensions::vec_extensions::VecExtensions;

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
        info!(
            "invoke started [{}={} {}={}]",
            name_of!(array_length in Self),
            self.array_length,
            name_of!(repeat_count in Self),
            self.repeat_count
        );

        let array = Vec::with_random_items_in_range(self.array_length, || 0..10000);
        let max = array.iter().max().ok_or(Error::msg("Failed to get max"))?;

        let mut results = Vec::with_capacity(self.repeat_count);

        for _ in 0..self.repeat_count {
            results.push(invoke_internal(&array)?)
        }

        let hit_count = results.iter().filter(|result| **result == *max).count();
        let hit_percent = (hit_count as f32 / self.repeat_count as f32) * 100.0;
        let average = results.iter().sum::<usize>() as f32 / results.len() as f32;

        info!(
            "invoke finished [{}={} {}={} {}={}]",
            name_of!(max),
            max,
            name_of!(hit_percent),
            hit_percent,
            name_of!(average),
            average
        );

        Ok(())
    }
}

fn invoke_internal(array: &Vec<usize>) -> Result<usize> {
    debug!("{} started", name_of!(invoke_internal));

    let n = array.len();
    let mut permutation = array.clone();
    permutation.shuffle(&mut rand::thread_rng());

    let threshold = permutation
        .iter()
        .take(n / 2)
        .max()
        .ok_or(Error::msg("failed to get maximum"))?;

    let result = permutation
        .iter()
        .skip(array.len() / 2)
        .filter(|item| **item >= *threshold)
        .next()
        .map_or(
            array.last().ok_or(Error::msg("no items in array"))?.clone(),
            |item| item.clone(),
        );

    debug!(
        "{} finished [{}={}]",
        name_of!(invoke_internal),
        name_of!(result),
        result
    );

    Ok(result)
}
