
pub mod selection_method {
    use rand::{RngCore, seq::SliceRandom};

    use crate::chromosome::chromosome::Chromosome;

    pub trait Individual {
        fn fitness(&self) -> f32;
        fn chromosome(&self) -> &Chromosome;
    }

    pub struct RouletteWheelSelection;

    impl RouletteWheelSelection {
        pub fn new() -> Self {
            Self
        }
    }

    pub trait SelectionMethod {
        fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
        where
            I: Individual;
    }

     impl SelectionMethod for RouletteWheelSelection {
        fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
        where
            I: Individual,
        {
            population
                .choose_weighted(rng, |individual| individual.fitness())
                .expect("Got an empty population")
        }
    }


#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

   

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteWheelSelection::new();
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];
        let actual_histogram: BTreeMap<i32, i32> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });

        let expected_histogram = maplit::btreemap! {
            1=>98,
            2=>202,
            3=>278,
            4=>422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}


}