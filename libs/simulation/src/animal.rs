use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,
    pub eye: Eye,
    pub brain: nn::Network,
}
impl Animal {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub(crate) fn random(rng: &mut dyn RngCore) -> Animal {
        let eye = Eye::default();
        let brain = nn::Network::random(
rng,
            &[
                nn::LayerTopology{
                    neurons:eye.cells(),
                },
                nn::LayerTopology{
                    neurons:2*eye.cells(),
                },
                nn::LayerTopology{
                    neurons:2,
                }
            ]
        );
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain
        }
    }
}
