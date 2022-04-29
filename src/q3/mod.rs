use anyhow::Result;
use clap::Args;
use log::warn;
use rand_distr::{Distribution, Normal};

use crate::{
    common::{
        algorithm::Algorithm, reduce::IntoReduce, repeat::IntoRepeat, with_name::IntoWithName,
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
        let array : Vec<f64> = Vec::with_random_items_in_range(1000, || 0..=2)
            .into_iter()
            .map(|num: i32| num as f64)
            .collect();

        JonsonLindenshtrassAlgorithm {
            input: JonsonLindenshtrassAlgorithmInput {
                n: 1000,
                epsilon: self.epsilon,
                delta: self.delta,
            },
        }
        .repeat(1000)
        .reduce(|series| {
            let l2_norm = array.l2_norm();
            let low_bar = (1.0 - self.epsilon) * l2_norm;
            let high_bar = (1.0 + self.epsilon) * l2_norm;

            let hit_count = series
                .iter()
                .filter(|matrix| match (*matrix).clone() * array.clone() {
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
        })
        .with_name("Q3 Algorithm".into())
        .run()
    }
}

#[derive(Debug, Clone)]
struct JonsonLindenshtrassAlgorithmInput {
    n: usize,
    epsilon: f64,
    delta: f64,
}

struct JonsonLindenshtrassAlgorithm {
    input: JonsonLindenshtrassAlgorithmInput,
}

impl Algorithm for JonsonLindenshtrassAlgorithm {
    type Input = JonsonLindenshtrassAlgorithmInput;
    type Output = Matrix<f64>;

    fn name(&self) -> String {
        "Jonson Lindenshtrass Algorithm".into()
    }

    fn input(&self) -> Self::Input {
        self.input.clone()
    }

    fn run_internal<F: Fn() + Send + Sync>(&self, update_progress: F) -> Result<Matrix<f64>> {
        let k = 21.0 * ((1.0 / self.input.delta).ln()) / self.input.epsilon.powi(2);

        let mut matrix = Matrix::new(self.input.n, k.ceil() as usize);

        let distribution = Normal::new(0.0, 1.0)?;
        let mut rng = rand::thread_rng();

        for mut row in matrix.iter_mut() {
            for item in row.iter_mut() {
                *item = distribution.sample(&mut rng);
            }
        }

        update_progress();

        Ok(matrix * (1.0 / k.sqrt()))
    }
}

#[derive(Debug)]
struct Q3AlgorithmResult {
    _hit_percent: f64,
}
