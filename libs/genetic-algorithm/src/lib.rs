#![feature(type_alias_impl_trait)]
mod chromosome;
use chromosome::chromosome::Chromosome;
use rand::{RngCore};
mod selection;
use crate::selection::selection_method::*;
mod crossover;
use crossover::crossover_method::*;
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
        }
    }
    pub fn evolve<I>(&self, population: &[I], rng: &mut dyn RngCore) -> Vec<I>
    where
        I: Individual,
    {
        // we cannot evolve an empty population
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                let child = self.crossover_method.crossover(
                    rng,
                    parent_a,
                    parent_b,
                );

                todo!()
            })
            .collect()
    }
}





