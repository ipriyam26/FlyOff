mod world;
use nalgebra as na;
use rand::{Rng, RngCore};
pub use world::{Animal, Food, World};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        /*
        ```
        Check if the the two circle's are colliding and if they are
        give the food a new location. i.e kill it and gen new one that's what it will look like to the users
        */

        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);
                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }
}
