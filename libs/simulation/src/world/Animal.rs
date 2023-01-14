use nalgebra as na;
use rand::{Rng, RngCore};
#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}
impl Animal {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub(crate) fn random(rng: &mut dyn RngCore) -> Animal {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }
}
