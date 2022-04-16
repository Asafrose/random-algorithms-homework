use std::{collections::HashMap, hash::Hash};
use rand::Rng;

pub struct HashFunction<TSourceItem: Hash + Eq,TTargetItem : Clone + Copy> {
    map: HashMap<TSourceItem,TTargetItem>,
}

impl<TSourceItem: Hash + Eq,TTargetItem : Clone + Copy> HashFunction<TSourceItem,TTargetItem> {
    pub fn new<TSource: IntoIterator<Item = TSourceItem>, TTarget: IntoIterator<Item = TTargetItem>>(
        source: TSource,
        target: TTarget,
    ) -> Self {
        let target = target.into_iter().collect::<Vec<TTargetItem>>();

        let mut thread_rng = rand::thread_rng();

        HashFunction {
            map: source
                .into_iter()
                .map(|item| (item, target[thread_rng.gen_range(0..target.len())]))
                .collect(),
        }
    }

    pub fn get_value(&self, x: &TSourceItem) -> TTargetItem {
        self.map.get(x).unwrap().clone()
    }
}
