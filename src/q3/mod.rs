use anyhow::{Result, Ok};
use clap::Args;
use rand_distr::{Normal, Distribution};

use crate::common::algorithm::Algorithm;

use self::matrix::Matrix;

mod matrix;

#[derive(Debug,Args)]
pub struct Q3Command;


impl Q3Command {
    pub fn invoke(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
struct JonsonLindenshtrassAlgorithmInput {
    n: usize,
    epsilon: f64,
    delta: f64
}

struct JonsonLindenshtrassAlgorithm;


impl Algorithm<JonsonLindenshtrassAlgorithmInput, Matrix<f64>> for JonsonLindenshtrassAlgorithm {
    fn name() -> String {
        "Jonson Lindenshtrass Algorithm".into()
    }

    fn run_internal(input: &JonsonLindenshtrassAlgorithmInput) -> Result<Matrix<f64>> {
        let k = 21.0 * ((1.0/input.delta).ln()) / input.epsilon.powi(2);

        let mut matrix = Matrix::new(input.n, k.ceil() as usize);

        let distribution = Normal::new(1.0 , 0.0)?;
        let mut rng = rand::thread_rng();

        for mut row in matrix.iter_mut() {
            for item in row.iter_mut() {
                *item = distribution.sample(&mut rng);
            }
        }
        
        Ok(matrix * (1.0/ k.sqrt()))
    }
}
