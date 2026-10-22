use super::knapsack::{Knapsack, Item};

pub fn greedy_approximation(all_items: &Vec<Item>, max_weight: u32) -> Knapsack {
    let mut all_items = all_items.clone();
    all_items.sort_by(|x, y| (y.value() as f64 / y.weight() as f64).total_cmp(&(x.value() as f64 / x.weight() as f64)));
    let mut res = Knapsack::new(max_weight);
    for item in all_items {
        res.add_item(&item);
    }
    res
}
