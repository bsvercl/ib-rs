use piston_window::types::Color;

use nphysics2d::object::RigidBodyHandle;

#[derive(Clone)]
pub struct Cuboid {
    pub body: RigidBodyHandle<f64>,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

impl Cuboid {
    pub fn new(width: f64, height: f64, color: Color, body: RigidBodyHandle<f64>) -> Self {
        let margin = body.borrow().margin();
        Cuboid {
            body: body,
            width: width + margin,
            height: height + margin,
            color: color,
        }
    }
}
