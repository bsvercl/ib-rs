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
mod color;
mod view;

fn main() {
    let mut app = app::App::new();
    app.run();
}
