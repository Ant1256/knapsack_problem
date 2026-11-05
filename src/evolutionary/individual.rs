//! Individual that is part of a complex evolutionary process
//!
//! Provides an individual that contains a knapsack filled with items. It can reproduce itself.
//! An individual has genes that are represented by a Knapsack
//! A gene of an individual is considered to be a basic element of a Knapsack : an Item.

use rand::RngExt;
use libm::exp;

use super::super::knapsack::{Knapsack, Item};
use super::Hyperparameters;

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
    fn modify_random_gene(&mut self, all_items: &Vec<Item>) {
        let mut rng = rand::rng();
        let addable_items = self.genes.addable_items(all_items);
        let withdrawable_items = self.genes.withdrawable_items(all_items);

        if addable_items.len() + withdrawable_items.len() == 0 {
            return;
        } 
        
        else if addable_items.len() == 0 {
            self.genes.withdraw_item(&withdrawable_items[rng.random_range(0..withdrawable_items.len())]);
        } 
        
        else if withdrawable_items.len() == 0 {
            self.genes.add_item(&addable_items[rng.random_range(0..addable_items.len())]);
        } 
        
        else {
            match rng.random_range(0..=1) {
                0 => {
                    self.genes.add_item(&addable_items[rng.random_range(0..addable_items.len())]);
                },
                _other => {
                    self.genes.withdraw_item(&withdrawable_items[rng.random_range(0..withdrawable_items.len())]);
                },
            }
        }
    }

    pub fn make_child(&self, hp: &Hyperparameters, all_items: &Vec<Item>) -> Self {
        let mut rng = rand::rng();
        let mut res = self.clone();
        while rng.random_range(0..hp.mut_chance) == 0 {
            res.modify_random_gene(all_items);
        };
        res
    }

    pub fn make_children(&self, hp: &Hyperparameters, all_items: &Vec<Item>) -> Vec<Self> {
        let mut res = Vec::new();
        let coef: f64 = ((exp(hp.growth_factor * self.genes.total_value() as f64 / hp.max_total_value as f64) - 1.0) / (exp(hp.growth_factor) - 1.0)) * hp.max_children as f64;
        let mut rng = rand::rng();
        let int_coef = coef as u32;
        let last_child_proba = coef - (int_coef as f64);
        for _ in 0..int_coef {
            res.push(self.make_child(hp, all_items));
        }
        if rng.random_range(0.0..1.0) < last_child_proba {
            res.push(self.make_child(hp, all_items));
        }
        res
    }
}
