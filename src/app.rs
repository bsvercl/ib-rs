use piston::input::{Key, MouseButton};
use graphics::Context;
use opengl_graphics::GlGraphics;

use rand::{self, Rng};

use nphysics2d::object::RigidBody;
use nphysics2d::world::World;
use ncollide;
use na;

use controller::{self, Controller};

pub struct App {
    current_controller: Box<Controller>,
}

// TODO: is there a better way to handle these?
impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        world.set_gravity(na::Vector2::new(0.0, 15.0));

        let rb = RigidBody::new_static(ncollide::shape::Plane2::new(na::Vector2::new(0.0, -1.0)),
                                       0.3,
                                       0.6);
        world.add_rigid_body(rb);

        let num = 55;
        let rad = 10.0;
        let shift = 2.5 * rad;
        let centerx = shift * (num as f64) / 2.0;

        for i in 0usize..num {
            for j in i..num {
                let fj = j as f64;
                let fi = i as f64;
                let x = (fi * shift / 2.0) + (fj - fi) * 2.5 * rad - centerx;
                let y = -fi * 2.5 * rad - 0.04 - rad;

                let mut rb = if rand::thread_rng().gen_range(0, 5) == 3 {
                    RigidBody::new_dynamic(ncollide::shape::Cuboid2::new(na::Vector2::new(rad -
                                                                                          0.04,
                                                                                          rad -
                                                                                          0.04)),
                                           1.0,
                                           0.3,
                                           0.6)
                } else {
                    RigidBody::new_dynamic(ncollide::shape::Ball2::new(rad - 0.04), 1.0, 0.3, 0.6)
                };
                rb.append_translation(&na::Translation2::new(x, y));
                world.add_rigid_body(rb);
            }
        }

        App { current_controller: Box::new(controller::Game::new(world)) }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_controller.update(dt);
    }

    pub fn render(&self, c: &Context, g: &mut GlGraphics) {
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
