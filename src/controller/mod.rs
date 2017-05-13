use piston::input::{Key, MouseButton};
use graphics::Context;
use opengl_graphics::GlGraphics;

mod game;

pub use self::game::Game;

pub trait Controller {
    fn update(&mut self, dt: f64);
    fn render(&self, c: &Context, g: &mut GlGraphics);

    fn handle_mouse_move(&mut self, _x: f64, _y: f64) {}
    fn handle_mouse_button(&mut self, _button: MouseButton, _pressed: bool) {}
    fn handle_mouse_scroll(&mut self, _x: f64, _y: f64) {}

    fn handle_key(&mut self, _key: Key, _pressed: bool) {}

    fn handle_resize(&mut self, _width: u32, _height: u32) {}
}
