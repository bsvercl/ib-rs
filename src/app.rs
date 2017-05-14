use piston::input::{Key, MouseButton};
use graphics::Context;
use opengl_graphics::GlGraphics;

use rand::{self, Rng};

use nphysics2d::detection::joint::{Anchor, BallInSocket};
use nphysics2d::object::RigidBody;
use nphysics2d::world::World;
use ncollide;
use ncollide::shape::{Ball2, Cuboid2, Plane2};
use na;

use controller::{self, Controller};

pub struct App {
    current_controller: Box<Controller>,
}

// TODO: is there a better way to handle these?
impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        world.set_gravity(na::Vector2::new(0.0, 30.0));

        let ground_geom = Plane2::new(na::Vector2::new(0.0, -1.0));
        world.add_rigid_body(RigidBody::new_static(ground_geom, 0.3, 0.6));

        let n = 5;
        let shift = 10.0;

        for i in 0usize..n {
            for j in 0usize..n {
                let x = i as f64 * shift - n as f64 * shift / 2.0;
                let y = j as f64 * (-shift) - 10.0;

                let pos = na::Vector2::new(x, y);

                // head
                let head_geom = Ball2::new(0.8);
                let mut head = RigidBody::new_dynamic(head_geom, 1.0, 0.3, 0.5);
                head.append_translation(&na::Translation2::from_vector(pos +
                                                                       na::Vector2::new(0.0,
                                                                                        -2.4)));

                // body
                let body_geom = Cuboid2::new(na::Vector2::new(1.2, 0.5));
                let mut body = RigidBody::new_dynamic(body_geom, 1.0, 0.3, 0.5);
                body.append_rotation(&na::UnitComplex::new(-::std::f64::consts::FRAC_2_PI));
                body.append_translation(&na::Translation2::from_vector(pos));

                // right arm
                let rarm_geom = Cuboid2::new(na::Vector2::new(1.6, 0.2));
                let mut rarm = RigidBody::new_dynamic(rarm_geom, 1.0, 0.3, 0.5);
                rarm.append_translation(&na::Translation2::from_vector(pos +
                                                                       na::Vector2::new(2.4,
                                                                                        -1.0)));

                // left arm
                let mut larm = rarm.clone();
                larm.append_translation(&na::Translation2::new(-4.8, 0.0));

                // right foot
                let rfoot_geom = Cuboid2::new(na::Vector2::new(1.6, 0.2));
                let mut rfoot = RigidBody::new_dynamic(rfoot_geom, 1.0, 0.3, 0.5);
                rfoot.append_rotation(&na::UnitComplex::new(-::std::f64::consts::FRAC_2_PI));
                rfoot.append_translation(&na::Translation2::from_vector(pos +
                                                                        na::Vector2::new(0.4,
                                                                                         3.0)));

                // left foot
                let mut lfoot = rfoot.clone();
                lfoot.append_translation(&na::Translation2::new(-0.8, 0.0));

                let head = world.add_rigid_body(head);
                let body = world.add_rigid_body(body);
                let rarm = world.add_rigid_body(rarm);
                let larm = world.add_rigid_body(larm);
                let rfoot = world.add_rigid_body(rfoot);
                let lfoot = world.add_rigid_body(lfoot);

                /*
                 * Create joints.
                 */
                let body_anchor_head = Anchor::new(Some(body.clone()), na::Point2::new(1.4, 0.0));
                let body_anchor_rarm = Anchor::new(Some(body.clone()), na::Point2::new(1.0, 0.76));
                let body_anchor_larm = Anchor::new(Some(body.clone()), na::Point2::new(1.0, -0.76));
                let body_anchor_rfoot = Anchor::new(Some(body.clone()), na::Point2::new(-1.5, 0.3));
                let body_anchor_lfoot = Anchor::new(Some(body.clone()),
                                                    na::Point2::new(-1.5, -0.3));

                let head_anchor = Anchor::new(Some(head), na::Point2::new(0.0, 0.9));
                let rarm_anchor = Anchor::new(Some(rarm), na::Point2::new(-1.5, 0.0));
                let larm_anchor = Anchor::new(Some(larm), na::Point2::new(1.5, 0.0));
                let rfoot_anchor = Anchor::new(Some(rfoot), na::Point2::new(1.5, 0.0));
                let lfoot_anchor = Anchor::new(Some(lfoot), na::Point2::new(1.5, 0.0));

                let head_joint = BallInSocket::new(body_anchor_head, head_anchor);
                let rarm_joint = BallInSocket::new(body_anchor_rarm, rarm_anchor);
                let larm_joint = BallInSocket::new(body_anchor_larm, larm_anchor);
                let rfoot_joint = BallInSocket::new(body_anchor_rfoot, rfoot_anchor);
                let lfoot_joint = BallInSocket::new(body_anchor_lfoot, lfoot_anchor);

                world.add_ball_in_socket(head_joint);
                world.add_ball_in_socket(rarm_joint);
                world.add_ball_in_socket(larm_joint);
                world.add_ball_in_socket(rfoot_joint);
                world.add_ball_in_socket(lfoot_joint);
            }
        }


        App { current_controller: Box::new(controller::Game::new(world)) }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_controller.update(dt);
    }

    pub fn render(&mut self, c: &Context, g: &mut GlGraphics) {
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
