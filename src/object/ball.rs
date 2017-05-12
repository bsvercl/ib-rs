use piston_window::types::Color;

use nphysics2d::object::RigidBodyHandle;

#[derive(Clone)]
pub struct Ball {
    pub body: RigidBodyHandle<f64>,
    pub radius: f64,
    pub color: Color,
}

impl Ball {
    pub fn new(radius: f64, color: Color, body: RigidBodyHandle<f64>) -> Self {
        let margin = body.borrow().margin();
        Ball {
            body: body,
            radius: radius + margin,
            color: color,
        }
    }
}
