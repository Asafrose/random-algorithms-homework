use std::{fs::File, io::Write};

use anyhow::{Ok, Result};
use clap::{Args, IntoApp};

use crate::command_line_arguments::CommandLineArguments;

#[derive(Debug, Args)]
pub struct DocumentCommand {
    #[clap(short, long)]
    path: String,
}

impl DocumentCommand {
    pub fn invoke(&self) -> Result<()> {
        let md = clap_md::app_to_md(&CommandLineArguments::command(), 1)
            .map_err(|e| anyhow::Error::msg(format!("{}", e)))?;

        let mut file = File::create(self.path.as_str())?;
        file.write_all(md.as_bytes())?;

        Ok(())
    }
}
