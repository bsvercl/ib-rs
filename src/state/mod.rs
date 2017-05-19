use piston_window::{Context, G2d, Key, MouseButton};

mod game;

pub use self::game::Game;

pub trait State {
    fn update(&mut self, dt: f64);
    fn render(&self, c: &Context, g: &mut G2d);

    fn handle_mouse_move(&mut self, _x: f64, _y: f64) {}
    fn handle_mouse_button(&mut self, _button: MouseButton, _pressed: bool) {}
    fn handle_mouse_scroll(&mut self, _x: f64, _y: f64) {}

    fn handle_key(&mut self, _key: Key, _pressed: bool) {}

    fn handle_resize(&mut self, _width: u32, _height: u32) {}
}
