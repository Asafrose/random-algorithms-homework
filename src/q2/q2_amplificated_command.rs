use anyhow::{Ok, Result};
use clap::Args;
use nameof::name_of;

use crate::{
    common::{
        algorithm::Algorithm, reduce::IntoReduce, repeat::IntoRepeat, with_name::IntoWithName,
    },
    extensions::vec_extensions::L2NormVecExtension,
};

use super::l2_algorithm::L2Algorithm;

#[derive(Debug, Args)]
pub struct Q2AmplificatedCommand {
    ///epsilon value
    #[clap(short, long, default_value = "0.5")]
    epsilon: f64,
    ///delta value
    #[clap(short, long, default_value = "0.01")]
    delta: f64,
}

impl Q2AmplificatedCommand {
    pub fn invoke(&self, array: Vec<f64>) -> Result<()> {
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
            L2Algorithm::new(array.clone())
                .repeat((9.0 / self.epsilon).ceil() as usize)
                .reduce(|series| Ok(series.iter().sum::<f64>() / series.len() as f64))
                .repeat((18.0 * (2.0 / self.delta).ln() + 1.0).floor() as usize)
                .reduce(|mut series| {
                    series.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let mid = series.len() / 2;
                    Ok(series[mid])
                })
                .repeat(1000)
                .reduce(move |series| {
                    let l2_norm: f64 = array.l2_norm();
                    let lower_bar = (1.0 - self.epsilon) * l2_norm;
                    let upper_bar = (1.0 + self.epsilon) * l2_norm;

                    let succession_count = series
                        .iter()
                        .filter(|num| **num >= lower_bar && **num <= upper_bar)
                        .count();

                    let succession_ratio = succession_count as f64 / series.len() as f64;

                    Ok(Q2AmplificatedAlgorithmResult {
                        _succession_ratio: succession_ratio * 100.0,
                    })
                })
                .with_name("Q2 Amplificated Algorithm".into())
                .run()
        }
    }
}

#[derive(Debug)]
struct Q2AmplificatedAlgorithmResult {
    _succession_ratio: f64,
}
