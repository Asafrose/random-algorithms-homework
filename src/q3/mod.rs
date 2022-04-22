use anyhow::Result;
use clap::Args;
use log::warn;
use rand_distr::{Distribution, Normal};

use crate::{
    common::{
        algorithm::Algorithm,
        repetition_algorithm::{RepetitionAlgorithm, RepetitionAlgorithmResult},
    },
    extensions::vec_extensions::{L2NormVecExtension, SampleUniformVecExtensions},
};

use self::matrix::Matrix;

mod matrix;

#[derive(Debug, Args)]
pub struct Q3Command {
    ///epsilon value
    #[clap(short, long, default_value = "0.5")]
    epsilon: f64,
    ///delta value
    #[clap(short, long, default_value = "0.01")]
    delta: f64,
}

impl Q3Command {
    pub fn invoke(&self) -> Result<()> {
        Q3Algorithm::run(
            crate::common::repetition_algorithm::RepetitionAlgorithmInput {
                input: Q3AlgorithmInput {
                    array: Vec::with_random_items_in_range(1000, || 0..=2)
                        .into_iter()
                        .map(|num| num as f64)
                        .collect(),
                    epsilon: self.epsilon,
                    delta: self.delta,
                },
                repetition_count: 1000,
            },
        )
    }
}

#[derive(Debug, Clone)]
struct JonsonLindenshtrassAlgorithmInput {
    n: usize,
    epsilon: f64,
    delta: f64,
}

struct JonsonLindenshtrassAlgorithm{
    input: JonsonLindenshtrassAlgorithmInput
}

impl Algorithm<JonsonLindenshtrassAlgorithmInput, Matrix<f64>> for JonsonLindenshtrassAlgorithm {
    fn name() -> String {
        "Jonson Lindenshtrass Algorithm".into()
    }

    fn run_internal(&self) -> Result<Matrix<f64>> {
        let k = 21.0 * ((1.0 / self.input.delta).ln()) / self.input.epsilon.powi(2);

        let mut matrix = Matrix::new(self.input.n, k.ceil() as usize);

        let distribution = Normal::new(0.0, 1.0)?;
        let mut rng = rand::thread_rng();

        for mut row in matrix.iter_mut() {
            for item in row.iter_mut() {
                *item = distribution.sample(&mut rng);
            }
        }

        Ok(matrix * (1.0 / k.sqrt()))
    }

    fn new(input: JonsonLindenshtrassAlgorithmInput, _is_update_progress: bool) -> Self {
        Self{
            input
        }
    }
}

#[derive(Debug)]
struct Q3AlgorithmResult {
    _hit_percent: f64,
}

impl RepetitionAlgorithmResult<Q3AlgorithmInput, Matrix<f64>> for Q3AlgorithmResult {
    fn new(input: &Q3AlgorithmInput, series: Vec<Matrix<f64>>) -> Result<Self> {
        let l2_norm = input.array.l2_norm();
        let low_bar = (1.0 - input.epsilon) * l2_norm;
        let high_bar = (1.0 + input.epsilon) * l2_norm;

        let hit_count = series
            .iter()
            .filter(|matrix| match (*matrix).clone() * input.array.clone() {
                Ok(vec) => {
                    let estimated_l2_norm = vec.l2_norm();

                    estimated_l2_norm >= low_bar && estimated_l2_norm <= high_bar
                }
                Err(err) => {
                    warn!("{}", err);
                    false
                }
            })
            .count();

        let hit_percent = (hit_count as f64 / series.len() as f64) * 100.0;

        Ok(Q3AlgorithmResult {
            _hit_percent: hit_percent,
        })
    }
}

#[derive(Debug, Clone)]
struct Q3AlgorithmInput {
    array: Vec<f64>,
    epsilon: f64,
    delta: f64,
}

struct Q3AlgorithmInner {
    inner: JonsonLindenshtrassAlgorithm,
}

impl Algorithm<Q3AlgorithmInput, Matrix<f64>> for Q3AlgorithmInner {
    fn name() -> String {
        JonsonLindenshtrassAlgorithm::name()
    }

    fn run_internal(&self) -> Result<Matrix<f64>> {
        self.inner.run_internal()
    }

    fn new(input: Q3AlgorithmInput, is_update_progress: bool) -> Self {
        Self {
            inner: JonsonLindenshtrassAlgorithm::new(
                JonsonLindenshtrassAlgorithmInput {
                    n: 1000,
                    epsilon: input.epsilon,
                    delta: input.delta,
                },
                is_update_progress
            ),
        }
    }
}

type Q3Algorithm =
    RepetitionAlgorithm<Q3AlgorithmInner, Q3AlgorithmResult, Q3AlgorithmInput, Matrix<f64>>;
