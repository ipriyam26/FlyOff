


mod world;
use rand::{RngCore};
use world::World;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn world (&self) -> &World{
    &self.world
    }
    
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
}
