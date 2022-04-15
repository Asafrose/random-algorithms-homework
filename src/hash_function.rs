use std::collections::HashMap;
use rand::Rng;

pub struct HashFunction {
    map: HashMap<i32, i32>,
}

impl HashFunction {
    pub fn new<TSource: IntoIterator<Item = i32>, TTarget: IntoIterator<Item = i32>>(
        source: TSource,
        target: TTarget,
    ) -> Self {
        let target = target.into_iter().collect::<Vec<i32>>();

        let mut thread_rng = rand::thread_rng();

        HashFunction {
            map: source
                .into_iter()
                .map(|item| (item, target[thread_rng.gen_range(0..target.len())]))
                .collect(),
        }
    }

    pub fn get_value(&self, x: i32) -> i32 {
        self.map.get(&x).unwrap().clone()
    }
}
