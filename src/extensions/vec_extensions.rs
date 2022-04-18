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



