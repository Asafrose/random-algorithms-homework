use anyhow::{Ok, Result};
use clap::Args;
use nameof::name_of;

use crate::{common::{
    algorithm::Algorithm,
    repetition_algorithm::{
        RepetitionAlgorithm, RepetitionAlgorithmInput, RepetitionAlgorithmResult,
    },
}, extensions::vec_extensions::L2NormVecExtension};

use super::l2_algorithm::L2Algorithm;

#[derive(Debug, Args)]
pub struct Q2AmplificatedCommand {
    #[clap(short, long)]
    epsilon: f64,
    #[clap(short, long)]
    delta: f64,
}

impl Q2AmplificatedCommand {
    pub fn invoke(&self, array: Vec<i32>) -> Result<()> {
        if self.delta < 0_f64 || self.delta > 1_f64 {
            Err(anyhow::Error::msg(format!(
                "delta must be between 0 and 1 [{}={}]",
                name_of!(delta in Self),
                self.delta
            )))
        } else if self.epsilon < 0_f64 || self.epsilon > 1_f64 {
            Err(anyhow::Error::msg(format!(
                "epsilon must be between 0 and 1 [{}={}]",
                name_of!(epsilon in Self),
                self.epsilon
            )))
        } else {
            Q2AmplificatedAlgorithm::run(Q2AmplificatedAlgorithmInput {
                array,
                epsilon: self.epsilon,
                delta: self.delta,
            })
        }
    }
}

#[derive(Debug, Clone)]
struct L2FirstAmplificationAlgorithmInput {
    array: Vec<i32>,
    epsilon: f64,
}

#[derive(Debug)]
struct L2FirstAmplificationAlgorithmResult {
    average: f64,
}

impl RepetitionAlgorithmResult<Vec<i32>, i32> for L2FirstAmplificationAlgorithmResult {
    fn new(_input: &Vec<i32>, series: Vec<i32>) -> Result<Self> {
        Ok(L2FirstAmplificationAlgorithmResult {
            average: series.iter().sum::<i32>() as f64 / series.len() as f64,
        })
    }
}

struct L2FirstAmplificationAlgorithm;

impl Algorithm<L2FirstAmplificationAlgorithmInput, L2FirstAmplificationAlgorithmResult>
    for L2FirstAmplificationAlgorithm
{
    fn name() -> String {
        "l2 first amplification algorithm".into()
    }

    fn run_internal(
        input: &L2FirstAmplificationAlgorithmInput,
    ) -> Result<L2FirstAmplificationAlgorithmResult> {
        RepetitionAlgorithm::<L2Algorithm, L2FirstAmplificationAlgorithmResult, Vec<i32>, i32>::run_internal(&RepetitionAlgorithmInput{
            input: input.array.clone(),
            repetition_count: (9.0/ input.epsilon).ceil() as usize
        })
    }
}

#[derive(Debug)]
struct L2SecondAmplificationAlgorithmInput {
    first_amplification_input: L2FirstAmplificationAlgorithmInput,
    delta: f64,
}

#[derive(Debug)]
struct L2SecondAmplificationAlgorithmResult {
    median: f64,
}

impl
    RepetitionAlgorithmResult<
        L2FirstAmplificationAlgorithmInput,
        L2FirstAmplificationAlgorithmResult,
    > for L2SecondAmplificationAlgorithmResult
{
    fn new(
        _input: &L2FirstAmplificationAlgorithmInput,
        series: Vec<L2FirstAmplificationAlgorithmResult>,
    ) -> Result<Self> {
        let mut numbers: Vec<f64> = series.iter().map(|item| item.average).collect();
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = numbers.len() / 2;
        let median = numbers[mid];

        Ok(L2SecondAmplificationAlgorithmResult { median })
    }
}

struct L2SecondAmplificationAlgorithm;

impl Algorithm<L2SecondAmplificationAlgorithmInput, L2SecondAmplificationAlgorithmResult>
    for L2SecondAmplificationAlgorithm
{
    fn name() -> String {
        "l2 second amplification algorithm".into()
    }

    fn run_internal(
        input: &L2SecondAmplificationAlgorithmInput,
    ) -> Result<L2SecondAmplificationAlgorithmResult> {
        RepetitionAlgorithm::<
            L2FirstAmplificationAlgorithm,
            L2SecondAmplificationAlgorithmResult,
            L2FirstAmplificationAlgorithmInput,
            L2FirstAmplificationAlgorithmResult,
        >::run_internal(&RepetitionAlgorithmInput {
            input: input.first_amplification_input.clone(),
            repetition_count: (18.0 * (2.0 / input.delta).ln() + 1.0).floor() as usize,
        })
    }
}

#[derive(Debug)]
struct Q2AmplificatedAlgorithmResult {
    _succession_ratio: f64,
}

impl
    RepetitionAlgorithmResult<
        L2SecondAmplificationAlgorithmInput,
        L2SecondAmplificationAlgorithmResult,
    > for Q2AmplificatedAlgorithmResult
{
    fn new(
        input: &L2SecondAmplificationAlgorithmInput,
        series: Vec<L2SecondAmplificationAlgorithmResult>,
    ) -> Result<Self> {
        let l2_norm: f64 =
            input.first_amplification_input.array.l2_norm().try_into()?;
        let epsilon = input.first_amplification_input.epsilon;

        let lower_bar = (1.0 - epsilon) * l2_norm;
        let upper_bar = (1.0 + epsilon) * l2_norm;

        let succession_count = series
            .iter()
            .map(|item| item.median)
            .filter(|num| *num >= lower_bar && *num <= upper_bar)
            .count();

        let succession_ratio = succession_count as f64 / series.len() as f64;

        Ok(Q2AmplificatedAlgorithmResult {
            _succession_ratio: succession_ratio,
        })
    }
}

#[derive(Debug)]
struct Q2AmplificatedAlgorithmInput {
    array: Vec<i32>,
    epsilon: f64,
    delta: f64,
}

struct Q2AmplificatedAlgorithm;

impl Algorithm<Q2AmplificatedAlgorithmInput, Q2AmplificatedAlgorithmResult>
    for Q2AmplificatedAlgorithm
{
    fn name() -> String {
        "q2 amplificated algorithm".into()
    }

    fn run_internal(input: &Q2AmplificatedAlgorithmInput) -> Result<Q2AmplificatedAlgorithmResult> {
        RepetitionAlgorithm::<
            L2SecondAmplificationAlgorithm,
            Q2AmplificatedAlgorithmResult,
            L2SecondAmplificationAlgorithmInput,
            L2SecondAmplificationAlgorithmResult,
        >::run_internal(&RepetitionAlgorithmInput {
            input: L2SecondAmplificationAlgorithmInput {
                first_amplification_input: L2FirstAmplificationAlgorithmInput {
                    array: input.array.clone(),
                    epsilon: input.epsilon,
                },
                delta: input.delta,
            },
            repetition_count: 1000,
        })
    }
}
