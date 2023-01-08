
mod animal;
use animal::Animal;
mod  food;
use food::Food;

// use food::Food;



#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}