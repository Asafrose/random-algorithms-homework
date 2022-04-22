use std::{fmt::Debug, marker::PhantomData};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::algorithm::Algorithm;

pub trait RepetitionAlgorithmResult<TInput, TOutput: Debug>: Debug + Sized {
    fn new(input: &TInput, series: Vec<TOutput>) -> Result<Self>;
}

pub struct RepetitionAlgorithm<
    TAlgorithm: Algorithm<TInput, TOutput>,
    TRepetitionAlgorithmResult: RepetitionAlgorithmResult<TInput, TOutput>,
    TInput: Debug + Clone,
    TOutput: Debug,
> {
    _phantom1: PhantomData<TRepetitionAlgorithmResult>,
    _phantom2: PhantomData<TOutput>,

    input: RepetitionAlgorithmInput<TInput>,
    algorithm: TAlgorithm,
    progress: ProgressBar,
}

#[derive(Debug, Clone)]
pub struct RepetitionAlgorithmInput<TInput: Debug + Clone> {
    pub input: TInput,
    pub repetition_count: usize,
}

impl<
        TAlgorithm: Algorithm<TInput, TOutput> + Sync + Send,
        TRepetitionAlgorithmResult: RepetitionAlgorithmResult<TInput, TOutput> + Send + Sync,
        TInput: Clone + Debug + Send + Sync,
        TOutput: Send + Debug + Sync,
    > Algorithm<RepetitionAlgorithmInput<TInput>, TRepetitionAlgorithmResult>
    for RepetitionAlgorithm<TAlgorithm, TRepetitionAlgorithmResult, TInput, TOutput>
{
    fn name() -> String {
        format!("{} repetition", TAlgorithm::name())
    }

    fn run_internal(&self) -> Result<TRepetitionAlgorithmResult> {
        let mut series = Vec::with_capacity(self.input.repetition_count);

        let progress = self.progress.clone();
        for item in (0..self.input.repetition_count)
            .into_par_iter()
            .map(move |_| {
                let result = self.algorithm.run_internal();
                progress.inc(1);
                result
            })
            .collect::<Vec<Result<TOutput>>>()
        {
            series.push(item?);
        }

        self.progress.finish_and_clear();

        TRepetitionAlgorithmResult::new(&self.input.input, series)
    }

    fn new(input: RepetitionAlgorithmInput<TInput>, is_update_progress: bool) -> Self {
        Self {
            algorithm: TAlgorithm::new(input.input.clone(), false),
            progress: ProgressBar::with_draw_target(
                input.repetition_count.try_into().unwrap(),
                if is_update_progress {
                    ProgressDrawTarget::stderr()
                } else {
                    ProgressDrawTarget::hidden()
                },
            )
            .with_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:100.green} {percent:>3}%")
                    .progress_chars("##-"),
            )
            .with_message(Self::name()),
            input,
            _phantom1: PhantomData::default(),
            _phantom2: PhantomData::default(),
        }
    }
}
