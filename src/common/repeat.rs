use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::algorithm::Algorithm;

pub struct Repeat<TAlgorithm> {
    algorithm: TAlgorithm,
    repetition_count: usize,
}

impl<TAlgorithm: Algorithm> Repeat<TAlgorithm> {
    fn new(algorithm: TAlgorithm, repetition_count: usize) -> Self {
        Self {
            algorithm,
            repetition_count,
        }
    }
}

impl<TAlgorithm: Algorithm + Sync + Send> Algorithm for Repeat<TAlgorithm> {
    fn name(&self) -> String {
        format!("{} repetition", self.algorithm.name())
    }

    type Input = (TAlgorithm::Input, usize);

    type Output = Vec<TAlgorithm::Output>;

    fn input(&self) -> Self::Input {
        (self.algorithm.input(), self.repetition_count)
    }

    fn get_repetitions(&self) -> u64 {
        self.algorithm.get_repetitions() * self.repetition_count as u64
    }

    fn run_internal<F: Fn() + Sync + Send>(&self, update_progress: F) -> Result<Self::Output> {
        let mut series = Vec::with_capacity(self.repetition_count);

        for item in (0..self.repetition_count)
            .into_par_iter()
            .map(move |_| {
                self.algorithm.run_internal(|| update_progress())
            })
            .collect::<Vec<Result<TAlgorithm::Output>>>()
        {
            series.push(item?);
        }

        Ok(series)
    }
}

pub trait IntoRepeat<TAlgorithm> {
    fn repeat(self, repetition_count: usize) -> Repeat<TAlgorithm>;
}

impl<TAlgorithm: Algorithm> IntoRepeat<TAlgorithm> for TAlgorithm {
    fn repeat(self, repetition_count: usize) -> Repeat<TAlgorithm> {
        Repeat::new(self, repetition_count)
    }
}
