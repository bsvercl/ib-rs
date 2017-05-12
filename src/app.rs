use std::cell::RefCell;
use std::rc::Rc;

use piston_window::{Context, G2d, Key, MouseButton};

use rand::{self, Rng};

use nphysics2d::detection::joint::{Anchor, Fixed};
use nphysics2d::object::{RigidBody, RigidBodyHandle, WorldObject};
use nphysics2d::world::World;
use ncollide;
use ncollide::world::CollisionGroups;
use na;

use camera::Camera;
use controller::{self, Controller};
use object::{Ball, Cuboid};

// TODO: refactor into smaller classes
pub struct App {
    world: World<f64>,
    balls: Vec<Ball>,
    cuboids: Vec<Cuboid>,
    camera: Camera,

    current_controller: Box<Controller>,

    grabbed_object: Option<RigidBodyHandle<f64>>,
    grabbed_object_joint: Option<Rc<RefCell<Fixed<f64>>>>,
    mouse_position: na::Vector2<f64>,

    move_camera_up: bool,
    move_camera_down: bool,
    move_camera_left: bool,
    move_camera_right: bool,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        let mut world = World::new();
        world.set_gravity(na::Vector2::new(0.0, 9.81));

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(na::Vector2::new(-1.0,
                                                                                        -1.0)),
                                           0.3,
                                           0.6);
        rb.append_translation(&na::Translation2::new(0.0, 10.0));
        world.add_rigid_body(rb);

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(na::Vector2::new(1.0,
                                                                                        -1.0)),
                                           0.3,
                                           0.6);
        rb.append_translation(&na::Translation2::new(0.0, 10.0));
        world.add_rigid_body(rb);

        let mut cuboids = vec![];
        let mut balls = vec![];

        let num = 5;
        let rad = 0.5;
        let shift = 2.5 * rad;
        let centerx = shift * (num as f64) / 2.0;
        let centery = shift * (num as f64) / 2.0;

        for i in 0usize..num {
            for j in 0usize..num {
                let x = i as f64 * 2.5 * rad - centerx;
                let y = j as f64 * 2.5 * rad - centery * 2.0 - 20.0;

                let r = rand::thread_rng().gen_range(0, 5) == 3;

                let mut rb = if r {
                    RigidBody::new_dynamic(ncollide::shape::Cuboid::new(na::Vector2::new(rad, rad)),
                                           1.0,
                                           0.3,
                                           0.6)
                } else {
                    RigidBody::new_dynamic(ncollide::shape::Ball::new(rad), 1.0, 0.3, 0.6)
                };

                rb.append_translation(&na::Translation2::new(x, y));
                let handle = world.add_rigid_body(rb);

                if r {
                    cuboids.push(Cuboid::new(rad, rad, [rand::random(), 1.0, 1.0, 1.0], handle));
                } else {
                    balls.push(Ball::new(rad, [rand::random(), 1.0, 1.0, 1.0], handle));
                }
            }
        }

        App {
            world: world,
            balls: balls,
            cuboids: cuboids,
            camera: Camera::new(width as f64, height as f64, 1.0),

            current_controller: Box::new(controller::Game::new()),

            grabbed_object: None,
            grabbed_object_joint: None,
            mouse_position: na::zero(),

            move_camera_up: false,
            move_camera_down: false,
            move_camera_left: false,
            move_camera_right: false,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_controller.update(dt);

        // TODO: magic number?
        let timestep = 1.0 / 60.0;

        self.world.step(timestep);

        let camera_move_speed: f64 = 50.0;

        let mut delta: na::Vector2<f64> = na::zero();

        if self.move_camera_up {
            delta.y = -camera_move_speed * dt;
        } else if self.move_camera_down {
            delta.y = camera_move_speed * dt;
        }

        if self.move_camera_left {
            delta.x = -camera_move_speed * dt;
        } else if self.move_camera_right {
            delta.x = camera_move_speed * dt;
        }

        self.camera.trans(delta);
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        self.current_controller.render(c, g);

        for ball in &self.balls {
            ball.render(&self.camera, c, g);
        }

        for cuboid in &self.cuboids {
            cuboid.render(&self.camera, c, g);
        }
    }
}

// input handlers
impl App {
    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_position = na::Vector2::new(x, y);

        let mapped_coords = self.camera.window_to_coord(self.mouse_position);
        let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);
        let attach2 = na::Isometry2::new(mapped_point.coords, 0.0);
        if let Some(_) = self.grabbed_object {
            let joint = self.grabbed_object_joint.as_ref().unwrap();
            joint.borrow_mut().set_local2(attach2);
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if pressed {
                let mapped_coords = self.camera.window_to_coord(self.mouse_position);
                let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);

                for b in self.world
                        .collision_world()
                        .interferences_with_point(&mapped_point, &CollisionGroups::new()) {
                    if let &WorldObject::RigidBody(ref rb) = &b.data {
                        if rb.borrow().can_move() {
                            self.grabbed_object = Some(rb.clone());
                        }
                    }
                }

                if let Some(ref b) = self.grabbed_object {
                    if let Some(ref j) = self.grabbed_object_joint {
                        self.world.remove_fixed(j);
                    }

                    let attach2 = na::Isometry2::new(mapped_point.coords, 0.0);
                    let attach1 = b.borrow().position().inverse() * attach2;
                    let anchor1 = Anchor::new(Some(b.clone()), attach1);
                    let anchor2 = Anchor::new(None, attach2);
                    let joint = Fixed::new(anchor1, anchor2);
                    self.grabbed_object_joint = Some(self.world.add_fixed(joint));
                }
            } else {
                if let Some(ref j) = self.grabbed_object_joint {
                    self.world.remove_fixed(j);
                }

                self.grabbed_object = None;
                self.grabbed_object_joint = None;
            }
        }
    }

    pub fn handle_mouse_scroll(&mut self, _: f64, y: f64) {
        self.camera.zoom += y;
    }

    pub fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::W => self.move_camera_up = pressed,
            Key::S => self.move_camera_down = pressed,
            Key::A => self.move_camera_left = pressed,
            Key::D => self.move_camera_right = pressed,
            _ => (),
        }
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.camera.set_size(width as f64, height as f64);
    }
}
