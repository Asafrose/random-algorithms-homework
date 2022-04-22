use std::fmt::Debug;

use anyhow::Result;
use log::info;
use nameof::name_of;

pub trait Algorithm<TInput: Debug + Clone, TOutput: Debug>: Sized {
    fn new(input: TInput, is_update_progress: bool) -> Self;

    fn name() -> String;
    fn run_internal(&self) -> Result<TOutput>;

    fn run(input: TInput) -> Result<()> {
        info!("{} started [{}={:?}]", Self::name(), name_of!(input), input);

        let algorithm = Self::new(input, true);

        let result = algorithm.run_internal()?;

        info!(
            "{} finished [{}={:?}]",
            Self::name(),
            name_of!(result),
            result
        );

        Ok(())
    }
}
