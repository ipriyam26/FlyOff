#![feature(type_alias_impl_trait)]
mod chromosome;
pub use chromosome::chromosome::Chromosome;
use rand::RngCore;
mod selection;
pub use crate::selection::selection_method::*;
mod crossover;
pub use crossover::crossover_method::*;
mod mutation;
pub use mutation::mutation_method::*;
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}
#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
}
impl Statistics {
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
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
    pub fn evolve<I>(&self, population: &[I], rng: &mut dyn RngCore) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        // we cannot evolve an empty population
        assert!(!population.is_empty());

        let new_population = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();
        let stats = Statistics::new(population);
        (new_population, stats)
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
            population = genaration.evolve(&population, &mut rng).0;
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
