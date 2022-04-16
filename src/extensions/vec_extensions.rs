use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

pub trait VecExtensions<TItem: SampleUniform> {
    fn with_random_items_in_range<TRange: SampleRange<TItem>, F: Fn() -> TRange>(
        length: usize,
        range: F,
    ) -> Self;
}

impl<TItem: SampleUniform> VecExtensions<TItem> for Vec<TItem> {
    fn with_random_items_in_range<TRange: SampleRange<TItem>, F: Fn() -> TRange>(
        length: usize,
        range: F,
    ) -> Self {
        let mut thread_rng = rand::thread_rng();

        (0..length).map(|_| thread_rng.gen_range(range())).collect()
    }
}
