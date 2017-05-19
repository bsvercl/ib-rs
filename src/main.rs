extern crate piston_window;

extern crate rand;
extern crate fps_counter;

extern crate nphysics2d;
extern crate ncollide;
extern crate nalgebra as na;

mod app;
mod camera;
mod color;
mod state;
mod view;

fn main() {
    let mut app = app::App::new();
    app.run();
}
