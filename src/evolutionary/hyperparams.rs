use super::super::knapsack::Item;

pub struct Hyperparameters {
    pub max_children: u32,
    pub growth_factor: f64,
    pub max_total_value: u32,
    pub mut_chance: usize,
}

impl Hyperparameters {
    pub fn new(max_children: u32, growth_factor: f64, all_items: &Vec<Item>, mut_chance: usize) -> Self {
        Self {
            max_children,
            growth_factor,
            max_total_value: sum_of_values(all_items),
            mut_chance,
        }
    }
}

fn sum_of_values(all_items: &Vec<Item>) -> u32 {
    let mut res: u32 = 0;
    for item in all_items {
        res += item.value();
    }
    res
}