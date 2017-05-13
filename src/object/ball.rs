use graphics::types::Color;

use nphysics2d::object::WorldObject;

#[derive(Clone)]
pub struct Ball {
    pub object: WorldObject<f64>,
    pub radius: f64,
    pub color: Color,
}

impl Ball {
    pub fn new(object: WorldObject<f64>, radius: f64, color: Color) -> Self {
        Ball {
            object: object,
            radius: radius,
            color: color,
        }
    }
}
