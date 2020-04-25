//! This module contains a logic for processing multiple solutions and multi objective optimization
//! based on NSGA2 algorithm.
//!
//! A NSGA2 implementation is based on the source code from the following repos:
//!
//! https://github.com/mneumann/dominance-ord-rs
//! https://github.com/mneumann/non-dominated-sort-rs
//! https://github.com/mneumann/nsga2-rs
//!
//! which is released under MIT License (MIT), copyright (c) 2016 Michael Neumann
//!

use crate::models::Problem;
use crate::solver::{Individual, Population};
use crate::utils::Random;
use std::sync::Arc;

mod crowding_distance;
use self::crowding_distance::*;

mod non_dominated_sort;
use self::non_dominated_sort::*;

mod nsga2;
use self::nsga2::select_and_rank;

/// An evolution aware implementation of `[Population]` trait.
pub struct DominancePopulation {
    problem: Arc<Problem>,
    random: Arc<dyn Random + Send + Sync>,
    individuals: Vec<Individual>,
    weights: Vec<usize>,
    max_size: usize,
}

impl DominancePopulation {
    /// Creates a new instance of `[EvoPopulation]`.
    pub fn new(problem: Arc<Problem>, random: Arc<dyn Random + Send + Sync>, max_size: usize) -> Self {
        Self {
            problem,
            random,
            individuals: vec![],
            weights: (0..max_size).map(|idx| max_size - idx).collect(),
            max_size,
        }
    }
}

impl Population for DominancePopulation {
    fn add(&mut self, individual: Individual) {
        self.individuals.push(individual);

        let mut best_order =
            select_and_rank(self.individuals.as_slice(), self.max_size, self.problem.objective.as_ref())
                .iter()
                .map(|acd| acd.index)
                .collect::<Vec<_>>();

        (0..best_order.len()).for_each(|i| loop {
            let j = best_order[i];
            let k = best_order[j];

            if i == j {
                break;
            }

            self.individuals.swap(j, k);
            best_order.swap(i, j);
        });

        self.individuals.truncate(self.max_size);
    }

    fn all<'a>(&'a self) -> Box<dyn Iterator<Item = &Individual> + 'a> {
        Box::new(self.individuals.iter())
    }

    fn best(&self) -> Option<&Individual> {
        self.individuals.first()
    }

    fn select(&self) -> &Individual {
        let idx = self.random.weighted(&self.weights[0..self.individuals.len()]);

        self.individuals.get(idx).unwrap()
    }

    fn size(&self) -> usize {
        self.individuals.len()
    }
}
