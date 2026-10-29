use std::collections::HashMap;

mod knapsack;
mod greedy_approximation;

use self::knapsack::{Knapsack, Item};
use self::greedy_approximation::greedy_approximation;

fn build_hashmap_from(items: &Vec<Item>) -> HashMap<String, Item> {
    let mut res = HashMap::new();
    for item in items {
        res.insert(item.name().clone(), item.clone());
    };
    res
}

fn main() {
    let all_items_vec: Vec<Item> = vec![
        Item::new("Livre", 1000, 15),
        Item::new("Carte", 200, 5),
        Item::new("Carte rare", 200, 25),
        Item::new("Lunettes", 500, 20),
        Item::new("Peluche", 500, 10),
        Item::new("Parfum", 800, 40),
        Item::new("Piece", 10, 10),
        Item::new("Portable", 1000, 100),
    ];
    let all_items = build_hashmap_from(&all_items_vec);

    {
        // Manipulation of Knapsack
        let mut knapsack_1 = Knapsack::new(2000);
        if
            knapsack_1.add_item(&all_items["Livre"]) &&
            knapsack_1.add_item(& all_items["Piece"]) &&
            knapsack_1.withdraw_item(& all_items["Piece"])
        {
            println!("L'ajout des items dans le sac s'est bien passé.\n");
        } else {
            println!("Impossible d'ajouter des items dans le sac.\n");
        }
        println!("Sac à dos de test après avoir rajouté des items :");
        knapsack_1.show(&all_items);
        println!("");
    }

    {
        // Greedy Algo
        let greedy: Knapsack = greedy_approximation(&all_items_vec, 2000);
        println!("Sac à dos optimal d'après l'algorithme glouton :");
        greedy.show(&all_items);
        println!("");
    }

}
