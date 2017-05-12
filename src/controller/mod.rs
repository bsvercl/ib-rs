use piston_window::{Context, G2d, Key, MouseButton};

use na;

mod game;
mod main_menu;

pub use self::game::Game;
pub use self::main_menu::MainMenu;

use camera::Camera;

pub trait Controller {
    fn update(&mut self, dt: f64);
    fn render(&self, c: &Context, g: &mut G2d);

    fn handle_mouse_move(&mut self, x: f64, y: f64) {}
    fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {}
    fn handle_mouse_scroll(&mut self, x: f64, y: f64) {}

    fn handle_key(&mut self, key: Key, pressed: bool) {}

    fn handle_resize(&mut self, width: u32, height: u32) {}
}
