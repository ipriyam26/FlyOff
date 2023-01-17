use crate::*;


#[derive(Debug)]
pub struct Food {
    pub position: na::Point2<f32>,
}
impl Food {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub(crate) fn random(rng: &mut dyn RngCore) -> Food {
        Self {
            position: rng.gen(),
        }
    }
}
