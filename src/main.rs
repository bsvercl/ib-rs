extern crate piston_window;
extern crate sdl2_window;

extern crate rand;
extern crate fps_counter;

extern crate nphysics2d;
extern crate ncollide;
extern crate nalgebra as na;

mod app;
mod camera;
mod controller;
mod draw;
mod color;
mod object;

use piston_window::{AdvancedWindow, EventLoop, PistonWindow, WindowSettings};
use piston_window::{Button, Input, Motion};
use sdl2_window::Sdl2Window;

use app::App;

fn main() {
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("", [512, 512])
        .vsync(true)
        .build()
        .unwrap();

    // set max updates to 60 for nphysics
    window.set_ups(60);

    let mut app = App::new();
    let mut counter = fps_counter::FPSCounter::new();

    while let Some(e) = window.next() {
        match e {
            Input::Update(args) => app.update(args.dt),

            Input::Render(_) => {
                window
                    .draw_2d(&e, |c, g| {
                        piston_window::clear(color::CORNFLOWER_BLUE, g);
                        app.render(&c, g);
                    })
                    .unwrap();
                window.set_title(format!("fps: {}", counter.tick()));
            }

            Input::Move(Motion::MouseCursor(x, y)) => app.handle_mouse_move(x, y),
            Input::Move(Motion::MouseScroll(x, y)) => app.handle_mouse_scroll(x, y),

            Input::Press(Button::Mouse(button)) => app.handle_mouse_button(button, true),
            Input::Release(Button::Mouse(button)) => app.handle_mouse_button(button, false),

            Input::Press(Button::Keyboard(key)) => app.handle_key(key, true),
            Input::Release(Button::Keyboard(key)) => app.handle_key(key, false),

            Input::Resize(width, height) => app.handle_resize(width, height),

            _ => {}
        }
    }
}
