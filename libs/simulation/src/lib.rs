// #![feature(crate_visibility_modifier)]

pub use self::{animal::*, eye::*, food::*, world::*};
use std::f32::consts::FRAC_PI_2;
mod animal;
mod eye;
mod food;
mod world;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.005;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

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
        self.process_brains();
        self.process_movements();
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.propogate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
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
