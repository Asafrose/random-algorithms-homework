use std::{fmt::Debug};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, debug};
use nameof::name_of;

pub trait Algorithm: Sized {
    type Input: Debug + Clone;
    type Output: Debug + Sync + Send;

    fn name(&self) -> String;
    fn input(&self) -> Self::Input;

    fn get_repetitions(&self) -> u64 {
        1
    }

    fn run_internal<F: Fn() + Sync + Send>(&self, update_progress: F) -> Result<Self::Output>;

    fn run(&self) -> Result<()> {
        info!("{} started", self.name());
        debug!("{}={:?}", "input", self.input());

        let progress = ProgressBar::new(self.get_repetitions())
            .with_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:100.green} {percent:>3}%")
                    .progress_chars("##-"),
            )
            .with_message(self.name());

        let result = self.run_internal(|| progress.inc(1))?;

        progress.finish_and_clear();
        info!(
            "{} finished [{}={:?}]",
            self.name(),
            name_of!(result),
            result
        );

        Ok(())
    }
}
