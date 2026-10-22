use std::collections::HashMap;

mod knapsack;

use self::knapsack::{Knapsack, Item};

fn main() {
    let knapsack_1 = Knapsack::new(50);
    let the_items: HashMap<String, Item> = HashMap::new();
    knapsack_1.show(&the_items);
}
