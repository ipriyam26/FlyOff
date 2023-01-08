mod animal;
use animal::Animal;
mod food;
use food::Food;
use rand::RngCore;

// use food::Food;

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}
impl World {
    pub fn animal(&self) -> &[Animal] {
        &self.animals
    }
    pub fn food(&self) -> &[Food] {
        &self.foods
    }

    pub(crate) fn random(rng: &mut dyn RngCore) -> World {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..40).map(|_| Food::random(rng)).collect();

        Self {
            animals: animals,
            foods: foods,
        }
    }
}
