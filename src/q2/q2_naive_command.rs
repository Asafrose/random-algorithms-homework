use clap::Args;
use anyhow::Result;
use log::info;
use nameof::name_of;

use super::{invoke_internal, get_l2_norm};

#[derive(Debug, Args)]
pub struct Q2NaiveCommand;

impl Q2NaiveCommand {
    pub fn invoke(&self, array: &Vec<i32>) -> Result<()> {
        info!("invoke started");
        let results: Vec<i32> = (0..1000).map(|_| invoke_internal(array)).collect();
        
        let average = results.iter().sum::<i32>() as f32 / results.len() as f32;
        let l2_norm = get_l2_norm(array);

        info!("invoke finished [{}={} {}={}]", name_of!(average), average, name_of!(l2_norm), l2_norm);

        Ok(())
    }
}