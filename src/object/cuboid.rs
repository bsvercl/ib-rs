use graphics::types::Color;

use nphysics2d::object::WorldObject;

#[derive(Clone)]
pub struct Cuboid {
    pub object: WorldObject<f64>,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

impl Cuboid {
    pub fn new(object: WorldObject<f64>, width: f64, height: f64, color: Color) -> Self {
        Cuboid {
            object: object,
            width: width,
            height: height,
            color: color,
        }
    }
}
