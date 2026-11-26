mod knapsack;
mod greedy_approximation;
mod evolutionary;
mod idgeneration;

use self::knapsack::{Knapsack, Item};
use self::greedy_approximation::greedy_approximation;
use self::evolutionary::{Individual, Hyperparameters, genetic_algorithm};
use self::idgeneration::IdGenerator;

fn main() {
    let mut id_generator = IdGenerator::new();

    let all_items: Vec<Item> = vec![
        Item::new(&mut id_generator, 1000, 15),
        Item::new(&mut id_generator, 200, 5),
        Item::new(&mut id_generator, 200, 25),
        Item::new(&mut id_generator, 500, 20),
        Item::new(&mut id_generator, 500, 10),
        Item::new(&mut id_generator, 800, 40),
        Item::new(&mut id_generator, 10, 10),
        Item::new(&mut id_generator, 1000, 100),
    ];

    {
        // Manipulation of Knapsack
        let mut knapsack_1 = Knapsack::new(2000);
        if
            knapsack_1.add_item(&all_items[3]) &&
            knapsack_1.add_item(&all_items[4]) &&
            knapsack_1.withdraw_item(& all_items[4])
        {
            println!("L'ajout des items dans le sac s'est bien passé.\n");
        } else {
            println!("Impossible d'ajouter des items dans le sac.\n");
        }
        println!("Sac à dos de test après avoir rajouté des items :");
        knapsack_1.show(&all_items);
        println!("");

        let mut the_hypers = Hyperparameters::new(&all_items);
        the_hypers.change_mut_chance(3);
        let the_indiv = Individual::from(&knapsack_1);
        let children = the_indiv.make_children(&the_hypers, &all_items);
        println!("Enfants générés avec 1/3 chance de muter : ");
        for child in children {
            child.show(&all_items);
            println!("");
        }
    }

    {
        // Greedy Algo
        let greedy: Knapsack = greedy_approximation(&all_items, 2000);
        println!("Sac à dos construit avec l'algorithme glouton :");
        greedy.show(&all_items);
        println!("");

        // Genetic algo
        let mut hyperparams = Hyperparameters::new(&all_items);
        hyperparams.change_total_iterations(1000);
        hyperparams.change_growth_factor(5.0);
        println!("Algorithme génétique avec logs : ");
        let genetic: Knapsack = genetic_algorithm(&all_items, 2000, &hyperparams, true);
        println!("");
        println!("Sac à dos construit avec l'algorithme génétique : ");
        println!("");
        genetic.show(&all_items);
    }

}
