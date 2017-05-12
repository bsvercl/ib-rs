use piston_window::{Context, G2d};

use super::Controller;

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }
}

impl Controller for Game {
    fn update(&mut self, _dt: f64) {}

    fn render(&self, _c: &Context, _g: &mut G2d) {}
}
