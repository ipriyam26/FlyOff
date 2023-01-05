pub mod crossover_method {

    use crate::Chromosome;

    use rand::{Rng, RngCore};

    pub trait CrossoverMethod {
        fn crossover(
            &self,
            rng: &mut dyn RngCore,
            parent_a: &Chromosome,
            parent_b: &Chromosome,
        ) -> Chromosome;
    }

    #[derive(Clone, Debug)]
    pub struct UniformCrossover;

    impl UniformCrossover {
        pub fn new() -> Self {
            Self
        }
    }

    impl CrossoverMethod for UniformCrossover {
        fn crossover(
            &self,
            rng: &mut dyn RngCore,
            parent_a: &Chromosome,
            parent_b: &Chromosome,
        ) -> Chromosome {
            let parent_a = parent_a.iter();
            let parent_b = parent_b.iter();

            parent_a
                .zip(parent_b)
                .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
                .collect()
        }
    }
    #[cfg(test)]
    mod tests {

        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
            let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

            let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

            let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
            let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();
            assert_eq!(diff_a, 49);
            assert_eq!(diff_b, 51);
        }
    }
}
