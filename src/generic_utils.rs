use std::collections::HashMap;

use super::Item;

pub fn build_hashmap_from(items: &Vec<Item>) -> HashMap<String, Item> {
    let mut res = HashMap::new();
    for item in items {
        res.insert(item.name().clone(), item.clone());
    };
    res
}
