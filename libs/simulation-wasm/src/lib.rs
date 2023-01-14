// use sim
use lib_simulation as sim;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);
        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        let animals = value.animal().iter().map(Animal::from).collect();
        Self { animals }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
            rotation: value.rotation().angle(),
        }
    }
}
