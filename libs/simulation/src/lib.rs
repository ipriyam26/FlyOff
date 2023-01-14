


mod world;
use rand::{RngCore};
pub use world::{World,Animal,Food};
use nalgebra as na;

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
    pub fn step(&mut self){
        for animal in &mut self.world.animals {
            animal.position += animal.rotation*na::Vector2::new(0.0, animal.speed);
        animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
        animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
