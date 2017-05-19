use color;
use fps_counter::FPSCounter;
use piston_window::{self, AdvancedWindow, Button, EventLoop, Input, Motion, PistonWindow,
                    WindowSettings};
use state::{self, State};

pub struct App {
    // Main window
    window: PistonWindow,

    current_controller: Box<State>,
}

impl App {
    pub fn new() -> Self {
        let mut window: PistonWindow = WindowSettings::new("", [800, 600])
            .samples(4)
            .build()
            .unwrap();
        window.set_ups(60);

        App {
            window: window,

            current_controller: Box::new(state::Game::new()),
        }
    }

    pub fn run(&mut self) {
        // Frame counter
        let mut counter = FPSCounter::new();

        // Main event loop
        while let Some(e) = self.window.next() {
            match e {
                Input::Update(ref args) => {
                    self.current_controller.update(args.dt);
                }

                Input::Render(_) => {
                    let ref mut current_controller = self.current_controller;
                    self.window
                        .draw_2d(&e, |c, g| {
                            piston_window::clear(color::CORNFLOWER_BLUE, g);
                            current_controller.render(&c, g);
                        })
                        .unwrap();
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
