#![feature(type_alias_impl_trait)]
mod chromosome;
use chromosome::chromosome::Chromosome;
use rand::RngCore;
mod selection;
use crate::selection::selection_method::*;
mod crossover;
use crossover::crossover_method::*;
mod mutation;
use mutation::mutation_method::MutationMethod;
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
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

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}
#[cfg(test)]

mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::mutation::mutation_method::GuassianMutation;

    use super::*;
    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let genaration = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GuassianMutation::new(0.5, 0.5),
        );
        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];
        for _ in 0..=10 {
            population = genaration.evolve(&population, &mut rng);
        }
        let expected_population = vec![
            individual(&[0.92582124, 1.5538777, 2.886911]),
            individual(&[-0.017893106, 2.0641832, 4.50793]),
            individual(&[0.71916926, 2.0648358, 4.3058133]),
            individual(&[0.6864883, 2.4618788, 4.024733]),
        ];
        assert_eq!(population, expected_population);
    }
}
