

#![feature(type_alias_impl_trait)]
use std::ops::Index;

use rand::{seq::SliceRandom, RngCore};

pub struct GeneticAlgorithm<S>{
    selection_method:S,
    
}

impl <S>GeneticAlgorithm<S>
where S:SelectionMethod
{
    pub fn new(selection_method:S) -> Self {
        Self{selection_method}
    }
    pub fn evolve<I>(&self, population: &[I],rng:&mut dyn RngCore) -> Vec<I> 
    where I:Individual,
    {
        // we cannot evolve an empty population
        assert!(!population.is_empty());

        (0..population.len()).map(|_| 
       { 
        let parent_a = self.selection_method.select(rng, population);
         let parent_b = self.selection_method.select(rng, population);
       todo!() }
        ).collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) ->&Chromosome;
}

pub trait CrossoverMethod{
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a:&Chromosome,
        parent_b :&Chromosome
    )-> Chromosome;
}

use rand::Rng;

#[derive(Clone,Debug)]
pub struct UniformCrossover;


impl UniformCrossover {
    pub fn new()->Self{
        Self
    }
}


impl CrossoverMethod for UniformCrossover{
    fn crossover(
            &self,
            rng: &mut dyn RngCore,
            parent_a:&Chromosome,
            parent_b :&Chromosome
        )-> Chromosome {
            let parent_a = parent_a.iter();
            let parent_b = parent_b.iter();
        
            parent_a
                .zip(parent_b)
                .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
                .collect()

    }
}




pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}


pub trait SelectionMethod {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual;
}

impl  SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Got an empty population")
    }
}


pub struct Chromosome{
    genes: Vec<f32>

}

impl Chromosome{
    pub fn len(&self)->usize    {
        self.genes.len()
    }

    pub fn iter(&self)->impl Iterator<Item=&f32>{
        self.genes.iter()
    }
    pub fn iter_mut(&mut self)->impl Iterator<Item = &mut f32>{
        self.genes.iter_mut()
    }
}
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}


impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
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

    fn chromosome(&self) ->&Chromosome {
        todo!()
    }
}



#[cfg(test)]
mod tests{
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::     ChaCha8Rng;

    use super::*;

    mod into_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            let genes: Vec<_> = chromosome.into_iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], 3.0);
            assert_eq!(genes[1], 1.0);
            assert_eq!(genes[2], 2.0);
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }
    mod from_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome: Chromosome =
                vec![3.0, 1.0, 2.0]
                    .into_iter()
                    .collect();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    #[test]
    fn test(){
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteWheelSelection::new();
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];
        let  actual_histogram: BTreeMap<i32, i32> = 
        (0..1000).map(|_| method.select(&mut rng, &population)).fold(Default::default(), |mut histogram,individual|{
            *histogram.entry( individual.fitness() as _).or_default()+=1;

            histogram
        });

        let expected_histogram = maplit::btreemap! {
            1=>98,
            2=>202,
            3=>278,
            4=>422,
        };

        assert_eq!(actual_histogram,expected_histogram);

    }

}