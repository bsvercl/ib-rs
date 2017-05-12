use piston_window::{Context, G2d};

mod game;
mod main_menu;

pub use self::game::Game;
pub use self::main_menu::MainMenu;

pub trait Controller {
    fn update(&mut self, dt: f64);
    fn render(&self, c: &Context, g: &mut G2d);
}
