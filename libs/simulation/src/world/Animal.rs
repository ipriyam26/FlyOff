use nalgebra as na;
#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Point2<f32>,
    speed: f32,
}
