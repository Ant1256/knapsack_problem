//! Individual that is part of a complex evolutionary process
//!
//! Provides an individual that contains a knapsack filled with items. It can reproduce itself.

use rand::RngExt;

use super::super::knapsack::{Knapsack, Item};

pub struct Individual {
    genes: Knapsack,
}

impl Clone for Individual {
    fn clone(&self) -> Self {
        Self {
            genes: self.genes.clone(),
        }
    }
}

impl Individual {
    pub fn make_child(&self, all_items: &Vec<Item>) -> Self {
        let mut rng = rand::rng();
        let mut res = self.clone();
        while rng.random_range(0..10) == 0 {
            modify_random_gene(&mut res.genes, all_items);
        };
        res
    }

    pub fn make_children(&self, max: usize) {}
}

fn modify_random_gene(the_knapsack: &mut Knapsack, all_items: &Vec<Item>) {
    let mut rng = rand::rng();
    let addable_items = the_knapsack.addable_items(all_items);
    let withdrawable_items = the_knapsack.withdrawable_items(all_items);

    if addable_items.len() + withdrawable_items.len() == 0 {
        return;
    } 
    
    else if addable_items.len() == 0 {
        the_knapsack.withdraw_item(&withdrawable_items[rng.random_range(0..withdrawable_items.len())]);
    } 
    
    else if withdrawable_items.len() == 0 {
        the_knapsack.add_item(&addable_items[rng.random_range(0..addable_items.len())]);
    } 
    
    else {
        match rng.random_range(0..=1) {
            0 => {
                the_knapsack.add_item(&addable_items[rng.random_range(0..addable_items.len())]);
            },
            _other => {
                the_knapsack.withdraw_item(&withdrawable_items[rng.random_range(0..withdrawable_items.len())]);
            },
        }
    }
}

