pub mod mutation_method {

    use crate::chromosome::chromosome::Chromosome;

    use rand::{RngCore, Rng};

    pub trait MutationMethod {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {}
    }

    pub struct GuassianMutation {
        pub(crate) chance: f32,
        pub(crate) coeff: f32,
    }

    impl GuassianMutation {
        pub fn new(chance: f32, coeff: f32) -> Self {
            assert!(chance >= 0.0 && chance <= 1.0);

            Self { chance, coeff }
        }
    }

    impl MutationMethod for GuassianMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            for gene in child.iter_mut() {
                let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                if rng.gen_bool(self.chance as _) {
                    *gene += sign * self.coeff * rng.gen::<f32>();
                }
            }
        }
    }
}
