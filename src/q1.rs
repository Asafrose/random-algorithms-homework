use anyhow::{Error, Result};
use clap::Args;
use nameof::name_of;

use crate::hash_function::HashFunction;

#[derive(Debug, Args)]
pub struct Q1Command {
    #[clap(short)]
    n: usize,
    #[clap(short, long)]
    array: String,
}

impl Q1Command {
    pub fn invoke(&self) -> Result<()> {
        let array = parse_array(&self.array, self.n.try_into()?)?;

        if array.len() != self.n {
            return Err(Error::msg(format!(
                "Incorrect array length [{}={} {}={}]",
                name_of!(n in Self),
                self.n,
                name_of!(array),
                array.len()
            )));
        }
        invoke_internal(&array)?;
        Ok(())
    }
}

fn invoke_internal(array: &Vec<i32>) -> Result<i32> {
    let n = array.len().try_into()?;
    let hash_function = HashFunction::new(0..n, 0..n);

    let threshold = array
        .iter()
        .take(array.len() / 2)
        .map(|x| hash_function.get_value(x.clone()))
        .max()
        .ok_or(Error::msg("failed to get maximum"))?;

    array
        .iter()
        .skip(array.len() / 2 + 1)
        .map(|item| hash_function.get_value(item.clone()))
        .filter(|item| item >= &threshold)
        .next()
        .map_or(
            Ok(hash_function.get_value(array.last().ok_or(Error::msg("no items in array"))?.clone())),
            |item| Ok(item),
        )
}

fn parse_array(array_string: &String, size: usize) -> Result<Vec<i32>> {
    let mut array = Vec::with_capacity(size);

    for item in array_string
        .split(",")
        .map(|token| token.trim())
        .map(|token| token.parse::<i32>())
    {
        array.push(item?);
    }

    Ok(array)
}
