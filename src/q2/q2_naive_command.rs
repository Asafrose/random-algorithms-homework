use anyhow::Result;
use clap::Args;

use crate::{
    common::{
        algorithm::Algorithm, reduce::IntoReduce, repeat::IntoRepeat, with_name::IntoWithName,
    },
    extensions::vec_extensions::L2NormVecExtension,
};

use super::l2_algorithm::L2Algorithm;

#[derive(Debug)]
pub struct Q2NaiveAlgorithmResult {
    _average: f64,
    _l2_norm: f64,
}

#[derive(Debug, Args)]
pub struct Q2NaiveCommand;

impl Q2NaiveCommand {
    pub fn invoke(&self, array: Vec<f64>) -> Result<()> {
        L2Algorithm::new(array.clone())
            .repeat(1000)
            .reduce(move |series| {
                let average = series.iter().sum::<f64>() / series.len() as f64;
                let l2_norm = array.l2_norm();

                Ok(Q2NaiveAlgorithmResult {
                    _average: average,
                    _l2_norm: l2_norm,
                })
            })
            .with_name("Q2 Naive Algorithm".into())
            .run()
    }
}
