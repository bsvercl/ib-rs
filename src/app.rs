use color;
use fps_counter::FPSCounter;
use glutin_window::GlutinWindow;
use graphics;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::event_loop::{EventLoop, Events, EventSettings};
use piston::input::{Button, Input, Motion};
use piston::window::{AdvancedWindow, WindowSettings};
use state::{self, State};


pub struct App {
    window: GlutinWindow,

    current_controller: Box<State>,
}

impl App {
    pub fn new() -> Self {
        App {
            window: WindowSettings::new("", [800, 600]).build().unwrap(),

            current_controller: Box::new(state::Game::new()),
        }
    }

    pub fn run(&mut self) {
        let mut counter = FPSCounter::new();

        let mut gl = GlGraphics::new(OpenGL::V3_2);

        let mut events = Events::new(EventSettings::new().ups(60));
        while let Some(e) = events.next(&mut self.window) {
            match e {
                Input::Update(ref args) => {
                    self.current_controller.update(args.dt);
                }

                Input::Render(ref args) => {
                    gl.draw(args.viewport(), |c, g| {
                        graphics::clear(color::CORNFLOWER_BLUE, g);
                        self.current_controller.render(&c, g);
                    });
                    self.window.set_title(format!("fps: {}", counter.tick()));
                }

                Input::Move(Motion::MouseCursor(x, y)) => {
                    self.current_controller.handle_mouse_move(x, y)
                }

                Input::Move(Motion::MouseScroll(x, y)) => {
                    self.current_controller.handle_mouse_scroll(x, y);
                }

                Input::Press(Button::Mouse(button)) => {
                    self.current_controller.handle_mouse_button(button, true)
                }

                Input::Release(Button::Mouse(button)) => {
                    self.current_controller.handle_mouse_button(button, false)
                }

                Input::Press(Button::Keyboard(key)) => {
                    self.current_controller.handle_key(key, true)
                }

                Input::Release(Button::Keyboard(key)) => {
                    self.current_controller.handle_key(key, false)
                }

                Input::Resize(width, height) => {
                    self.current_controller.handle_resize(width, height)
                }

                _ => {}
            }
        }
    }
}
