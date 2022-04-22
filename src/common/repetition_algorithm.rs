use std::{fmt::Debug, marker::PhantomData};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::algorithm::Algorithm;

pub trait RepetitionAlgorithmResult<TInput, TOutput: Debug>: Debug + Sized {
    fn new(input: &TInput, series: Vec<TOutput>) -> Result<Self>;
}

pub struct RepetitionAlgorithm<
    TAlgorithm: Algorithm<TInput, TOutput>,
    TRepetitionAlgorithmResult: RepetitionAlgorithmResult<TInput, TOutput>,
    TInput: Debug,
    TOutput: Debug,
> {
    _phantom1: PhantomData<TAlgorithm>,
    _phantom2: PhantomData<TRepetitionAlgorithmResult>,
    _phantom3: PhantomData<TInput>,
    _phantom4: PhantomData<TOutput>,
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
        let progress_bar = ProgressBar::new(input.repetition_count.try_into()?)
            .with_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .progress_chars("##-"),
            )
            .with_message(Self::name());

        for item in (0..input.repetition_count)
            .into_par_iter()
            .map(|_| {
                let result = TAlgorithm::run_internal(&input.input);
                progress_bar.inc(1);
                result
            })
            .collect::<Vec<Result<TOutput>>>()
        {
            series.push(item?);
        }

        progress_bar.finish_and_clear();

        TRepetitionAlgorithmResult::new(&input.input, series)
    }
}
