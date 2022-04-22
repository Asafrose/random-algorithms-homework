use anyhow::{Result, Ok};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

pub trait SampleUniformVecExtensions<TItem: SampleUniform> {
    fn with_random_items_in_range<TRange: SampleRange<TItem>, F: Fn() -> TRange>(
        length: usize,
        range: F,
    ) -> Self;
}

impl<TItem: SampleUniform> SampleUniformVecExtensions<TItem> for Vec<TItem> {
    fn with_random_items_in_range<TRange: SampleRange<TItem>, F: Fn() -> TRange>(
        length: usize,
        range: F,
    ) -> Self {
        let mut thread_rng = rand::thread_rng();

        (0..length).map(|_| thread_rng.gen_range(range())).collect()
    }
}

pub trait L2NormVecExtension<TResult> {
    fn l2_norm(&self) -> TResult;
}

impl L2NormVecExtension<i32> for Vec<i32> {
    fn l2_norm(&self) -> i32 {
        self.iter().map(|num| num.pow(2)).sum()
    }
}

impl L2NormVecExtension<f64> for Vec<f64> {
    fn l2_norm(&self) -> f64 {
        self.iter().map(|num| num.powi(2)).sum()
    }
}

pub trait Single<TResult> {
    fn single(self) -> Result<TResult>;
}

impl<TItem : Copy> Single<TItem> for Vec<TItem> {
    fn single(self) -> Result<TItem> {
        match self.len() {
            0 => Err(anyhow::Error::msg("Sequence contains no elements")),
            1 => Ok(self[0]),
            _ => Err(anyhow::Error::msg("sequence contains multiple elements"))
        }
    }
}

pub trait TryCollect<TResult> {
    fn try_collect(self) -> Result<Vec<TResult>>;
}

impl<TItem, TIterator : Iterator<Item = Result<TItem>>> TryCollect<TItem> for TIterator {
    fn try_collect(self) -> Result<Vec<TItem>> {
        let mut result = Vec::new();
    
        for item in self {
            result.push(item?);
        }

        Ok(result)
    }
}



