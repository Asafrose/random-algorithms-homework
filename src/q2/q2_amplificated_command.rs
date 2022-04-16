use clap::Args;
use anyhow::Result;

#[derive(Debug, Args)]
pub struct Q2AmplificatedCommand {
    epsilon: f32,
    delta: f32
}

impl Q2AmplificatedCommand {
    pub fn invoke(&self) -> Result<()> {
        Ok(())
    }
}