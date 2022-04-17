use std::fmt::Debug;

use anyhow::{Ok, Result};
use log::info;
use nameof::name_of;

pub trait Algorithm<TInput: Debug, TOutput: Debug> {
    fn name() -> String;
    fn run_internal(input: &TInput) -> Result<TOutput>;

    fn run(input: TInput) -> Result<()> {
        info!("{} started [{}={:?}]", Self::name(), name_of!(input), input);

        let result = Self::run_internal(&input)?;

        info!(
            "{} finished [{}={:?}]",
            Self::name(),
            name_of!(result),
            result
        );

        Ok(())
    }
}
