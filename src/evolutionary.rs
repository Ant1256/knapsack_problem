// use rand::RngExt;
use rand::seq::SliceRandom;

mod individual;
mod hyperparams;

pub use individual::Individual;
pub use hyperparams::Hyperparameters;
use super::{Item, Knapsack, greedy_approximation};

pub fn genetic_algorithm(all_items: &Vec<Item>, max_weight: u32, hp: &Hyperparameters) -> Knapsack {
    let start_knapsack = greedy_approximation(all_items, max_weight);
    let start = Individual::from(&start_knapsack);
    let mut population: Vec<Individual> = vec![start];
    let mut next_generation: Vec<Individual>;
    for iter_num in 0..hp.total_iterations {
        next_generation = Vec::new();
        for indiv in population {
            next_generation.append(&mut indiv.make_children(hp, all_items));
        }
        population = next_generation;
        shrink_population(&mut population, hp.max_individuals);
    }
    mode(&population).genes().clone()
}

fn shrink_population(population: &mut Vec<Individual>, max_amount: usize) {
    let mut rng = rand::rng();
    population.shuffle(&mut rng);
    *population = population[..max_amount].to_vec();
}

fn mode(population: &Vec<Individual>) -> Individual {
    if population.len() == 0 {
        panic!("Population must contain at least an individual.");
    }
    let mut count: Vec<(&Individual, u32)> = Vec::new();
    let mut found: bool;
    for indiv in population {
        found = false;
        for c_indiv in &mut count {
            if indiv == c_indiv.0 {
                *c_indiv = (c_indiv.0, c_indiv.1 + 1);
                found = true;
            }
        }
        if !found {
            count.push((&indiv, 1));
        }
    }
    let max_key: Option<(&Individual, u32)> = count.iter().fold(None, |best, (indiv, val)| match best {
        None => Some((indiv, *val)),
        Some(thing) if *val > thing.1 => Some((indiv, *val)),
        other => other,
    });
    max_key.unwrap().0.clone()
}


