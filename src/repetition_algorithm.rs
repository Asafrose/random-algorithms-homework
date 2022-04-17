use std::{fmt::Debug, marker::PhantomData};

use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::algorithm::Algorithm;

pub trait RepetitionAlgorithmResult<TInput, TOutput: Debug>: Debug + Sized {
    fn new(input: &TInput, series: Vec<TOutput>) -> Result<Self>;
}

pub struct RepetitionAlgorithm<
    TAlgorithm: Algorithm<TInput, TOutput>,
    TRepetitionAlgorithmResult: RepetitionAlgorithmResult<TInput, TOutput>,
    TInput: Debug,
    TOutput: Debug,
> {
    phantom1: PhantomData<TAlgorithm>,
    phantom2: PhantomData<TRepetitionAlgorithmResult>,
    phantom3: PhantomData<TInput>,
    phantom4: PhantomData<TOutput>,
}

#[derive(Debug)]
pub struct RepetitionAlgorithmInput<TInput: Debug> {
    pub input: TInput,
    pub repetition_count: usize,
}

impl<
        TAlgorithm: Algorithm<TInput, TOutput>,
        TRepetitionAlgorithmResult: RepetitionAlgorithmResult<TInput, TOutput>,
        TInput: Debug + Send + Sync,
        TOutput: Send + Debug,
    > Algorithm<RepetitionAlgorithmInput<TInput>, TRepetitionAlgorithmResult>
    for RepetitionAlgorithm<TAlgorithm, TRepetitionAlgorithmResult, TInput, TOutput>
{
    fn name() -> String {
        format!("{} repetition", TAlgorithm::name())
    }

    fn run_internal(
        input: &RepetitionAlgorithmInput<TInput>,
    ) -> Result<TRepetitionAlgorithmResult> {
        let mut series = Vec::with_capacity(input.repetition_count);

        for item in (0..input.repetition_count)
            .into_par_iter()
            .map(|_| TAlgorithm::run_internal(&input.input))
            .collect::<Vec<Result<TOutput>>>()
        {
            series.push(item?);
        }

        TRepetitionAlgorithmResult::new(&input.input, series)
    }
}
