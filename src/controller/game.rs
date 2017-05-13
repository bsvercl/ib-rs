use std::cell::RefCell;
use std::rc::Rc;

use piston::input::{Key, MouseButton};
use graphics::Context;
use opengl_graphics::GlGraphics;

use rand;

use nphysics2d::detection::joint::{Anchor, Fixed};
use nphysics2d::object::{RigidBody, RigidBodyHandle, WorldObject};
use nphysics2d::world::World;
use ncollide;
use ncollide::world::CollisionGroups;
use na;

use super::Controller;

use camera::Camera;
use draw::Draw;
use object::{Ball, Cuboid};

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum Action {
    CreatingBall,
    CreatingCuboid,
    CreatingTriangle,
    CreatingFixedJoint,

    Rotating,
    Paste,

    BoxSelecting,

    CreatingText,
    ResizingText,

    None,
}

pub struct Game {
    world: World<f64>,
    draw: Draw,
    camera: Camera,

    balls: Vec<Ball>,
    cuboids: Vec<Cuboid>,

    grabbed_object: Option<RigidBodyHandle<f64>>,
    grabbed_object_joint: Option<Rc<RefCell<Fixed<f64>>>>,
    mouse_position: na::Vector2<f64>,

    current_action: Action,
    action_step: i32,
    first_click: na::Vector2<f64>,

    move_camera_up: bool,
    move_camera_down: bool,
    move_camera_left: bool,
    move_camera_right: bool,
}

impl Game {
    pub fn new_empty() -> Self {
        Game {
            world: World::new(),
            draw: Draw::new(),
            camera: Camera::new(800.0, 600.0),

            balls: vec![],
            cuboids: vec![],

            grabbed_object: None,
            grabbed_object_joint: None,
            mouse_position: na::zero(),

            current_action: Action::None,
            action_step: 0,
            first_click: na::zero(),

            move_camera_up: false,
            move_camera_down: false,
            move_camera_left: false,
            move_camera_right: false,
        }

    }

    pub fn new(world: World<f64>) -> Self {
        let mut game = Game::new_empty();
        game.set_world(world);

        game
    }

    fn set_world(&mut self, world: World<f64>) {
        self.world = world;

        for rb in self.world.rigid_bodies() {
            let object = WorldObject::RigidBody(rb.clone());
            let bobject = object.borrow();
            let shape = bobject.shape().as_ref();
            let margin = bobject.margin();

            if let Some(s) = shape.as_shape::<ncollide::shape::Ball2<f64>>() {
                self.balls
                    .push(Ball::new(object.clone(),
                                    s.radius() + margin,
                                    [rand::random(), rand::random(), rand::random(), 1.0]));
            } else if let Some(s) = shape.as_shape::<ncollide::shape::Cuboid2<f64>>() {
                self.cuboids
                    .push(Cuboid::new(object.clone(),
                                      s.half_extents().x + margin,
                                      s.half_extents().y + margin,
                                      [rand::random(), rand::random(), rand::random(), 1.0]));
            }
        }
    }

    fn trans_camera(&mut self, dt: f64) {
        let camera_move_speed = 50.0;

        let mut delta: na::Vector2<f64> = na::zero();

        if self.move_camera_up {
            delta.y = -camera_move_speed;
        } else if self.move_camera_down {
            delta.y = camera_move_speed;
        }

        if self.move_camera_left {
            delta.x = -camera_move_speed;
        } else if self.move_camera_right {
            delta.x = camera_move_speed;
        }

        self.camera.trans(delta * dt);
    }
}

impl Controller for Game {
    fn update(&mut self, dt: f64) {
        let timestep = 1.0 / 60.0;
        self.world.step(timestep);

        self.trans_camera(dt);
    }

    fn render(&self, c: &Context, g: &mut GlGraphics) {
        for ball in &self.balls {
            self.draw.render_ball(&ball, &self.camera, c, g);
        }

        for cuboid in &self.cuboids {
            self.draw.render_cuboid(&cuboid, &self.camera, c, g);
        }

        if self.action_step == 1 {
            match self.current_action {
                Action::CreatingBall => {
                    self.draw
                        .render_temp_ball(self.first_click, self.mouse_position, &self.camera, c, g)
                }
                Action::CreatingCuboid => {
                    self.draw
                        .render_temp_cuboid(self.first_click,
                                            self.mouse_position,
                                            &self.camera,
                                            c,
                                            g)
                }
                _ => (),
            }
        }
    }

    fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_position = na::Vector2::new(x, y);

        let mapped_coords = self.camera.window_to_coord(self.mouse_position);
        let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);
        let attach2 = na::Isometry2::new(mapped_point.coords, 0.0);
        if let Some(_) = self.grabbed_object {
            let joint = self.grabbed_object_joint.as_ref().unwrap();
            joint.borrow_mut().set_local2(attach2);
        }
    }

    fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if self.current_action == Action::None {
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
            } else if self.current_action == Action::CreatingBall {
                if pressed && self.action_step == 0 {
                    self.first_click = self.mouse_position;
                    self.action_step += 1;
                } else if !pressed && self.action_step == 1 {
                    let first_click = self.camera.window_to_coord(self.first_click);
                    let mapped_first_click = na::Point2::new(first_click.x, first_click.y);
                    let mouse_position = self.camera.window_to_coord(self.mouse_position);
                    let mapped_mouse_position = na::Point2::new(mouse_position.x, mouse_position.y);

                    let radius = na::distance(&mapped_first_click, &mapped_mouse_position);
                    let radius = if radius < 0.1 {
                        0.1
                    } else if radius > 10.0 {
                        10.0
                    } else {
                        radius
                    };
                    if radius > 0.0 {
                        self.current_action = Action::None;

                        let pos = self.camera.window_to_coord(self.first_click);

                        let ball = ncollide::shape::Ball2::new(radius);
                        let mut rb = RigidBody::new_dynamic(ball, 1.0, 0.3, 0.6);
                        rb.append_translation(&na::Translation2::new(pos.x, pos.y));
                        let handle = self.world.add_rigid_body(rb);
                        self.balls
                            .push(Ball::new(WorldObject::RigidBody(handle.clone()),
                                            radius,
                                            [1.0; 4]));
                    }
                }
            } else if self.current_action == Action::CreatingCuboid {
                if pressed && self.action_step == 0 {
                    self.first_click = self.mouse_position;
                    self.action_step += 1;
                } else if !pressed && self.action_step == 1 {
                    if self.mouse_position != self.first_click {
                        self.current_action = Action::None;

                        let pos = self.camera.window_to_coord(self.first_click);
                        let width = self.mouse_position.x - self.first_click.x;
                        let width = if width < 0.1 {
                            0.1
                        } else if width > 10.0 {
                            10.0
                        } else {
                            width
                        };
                        let height = self.mouse_position.y - self.first_click.y;
                        let height = if height < 0.1 {
                            0.1
                        } else if height > 10.0 {
                            10.0
                        } else {
                            height
                        };

                        let cuboid = ncollide::shape::Cuboid2::new(na::Vector2::new(width, height));
                        let mut rb = RigidBody::new_dynamic(cuboid, 1.0, 0.3, 0.6);
                        rb.append_translation(&na::Translation2::new(pos.x, pos.y));
                        let handle = self.world.add_rigid_body(rb);
                        self.cuboids
                            .push(Cuboid::new(WorldObject::RigidBody(handle.clone()),
                                              width,
                                              height,
                                              [1.0; 4]));
                    }
                }
            }
        }
    }

    fn handle_mouse_scroll(&mut self, _: f64, y: f64) {
        let zoom = self.camera.zoom();
        self.camera.set_zoom(zoom + y * 0.05);
    }

    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Up => self.move_camera_up = pressed,
            Key::Down => self.move_camera_down = pressed,
            Key::Left => self.move_camera_left = pressed,
            Key::Right => self.move_camera_right = pressed,

            Key::D1 if pressed => {
                self.current_action = Action::CreatingBall;
                self.action_step = 0;
            }

            Key::D2 if pressed => {
                self.current_action = Action::CreatingCuboid;
                self.action_step = 0;
            }

            _ => (),
        }
    }

    fn handle_resize(&mut self, width: u32, height: u32) {
        self.camera.set_size(width as f64, height as f64);
    }
}
