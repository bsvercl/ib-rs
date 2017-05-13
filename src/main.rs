extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

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

use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, Input, Motion};
use piston::window::{AdvancedWindow, WindowSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow;

use app::App;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("", [800, 600])
        .opengl(opengl)
        .build()
        .unwrap();

    let mut app = App::new();
    let mut counter = fps_counter::FPSCounter::new();
    let mut fps = 0;

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Update(args) => {
                app.update(args.dt);
                window.set_title(format!("fps: {}", fps));
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |c, g| {
                    graphics::clear(color::CORNFLOWER_BLUE, g);
                    app.render(&c, g);
                });
                fps = counter.tick();
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
