use piston_window::{Context, G2d};

use super::Controller;

pub struct MainMenu {}

impl Controller for MainMenu {
    fn update(&mut self, dt: f64) {}

    fn render(&self, c: &Context, g: &mut G2d) {}
}
