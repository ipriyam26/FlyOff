pub struct  GeneticAlgorithm;

impl GenetricAlgorithm {
    pub fn new() -> Self{
        Self
    }
    pub fn evolve<I>(&self,population:&[I])->Vec<I>{
        // we cannot evolve an empty population
        assert!(!population.is_empty());

        (0..population.len())
        .map(|_|{
            todo!()
        }).collect()

    }
}