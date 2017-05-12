use piston_window::{Context, G2d, Key, MouseButton};

use controller::{self, Controller};

pub struct App {
    current_controller: Box<Controller>,
}

// TODO: is there a better way to handle these?
impl App {
    pub fn new() -> Self {
        App { current_controller: Box::new(controller::Game::new()) }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_controller.update(dt);
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        self.current_controller.render(c, g);
    }

    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.current_controller.handle_mouse_move(x, y);
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        self.current_controller.handle_mouse_button(button, pressed);
    }

    pub fn handle_mouse_scroll(&mut self, x: f64, y: f64) {
        self.current_controller.handle_mouse_scroll(x, y);
    }

    pub fn handle_key(&mut self, key: Key, pressed: bool) {
        self.current_controller.handle_key(key, pressed);
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.current_controller.handle_resize(width, height);
    }
}
