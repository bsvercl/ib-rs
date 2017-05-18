use super::State;
use camera::Camera;
use color;
use graphics::{self, Context, Transformed};
use na;
use ncollide::shape::{Ball2, Cuboid2, Plane2};
use ncollide::world::CollisionGroups;
use nphysics2d::detection::constraint::Constraint;
use nphysics2d::detection::joint::{Anchor, Fixed, Joint};
use nphysics2d::object::{RigidBody, RigidBodyHandle, WorldObject};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;
use piston::input::{Key, MouseButton};
use std::cell::RefCell;
use std::rc::Rc;
use view;

const MAX_CUBOID_WIDTH: f64 = 10.0;
const MIN_CUBOID_WIDTH: f64 = 0.1;

const MAX_CUBOID_HEIGHT: f64 = 10.0;
const MIN_CUBOID_HEIGHT: f64 = 0.1;

const MAX_BALL_RADIUS: f64 = 10.0;
const MIN_BALL_RADIUS: f64 = 0.1;

const MIN_ZOOM: f64 = 12.0;
const MAX_ZOOM: f64 = 75.0;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum Action {
    CreatingBall,
    CreatingCuboid,
    CreatingTriangle,
    CreatingFixedJoint,
    CreatingBallInSocket,

    Rotating,
    Paste,

    BoxSelecting,

    CreatingText,
    ResizingText,

    None,
}

pub struct Game {
    world: World<f64>,
    // Handles conversions between world->window and window->world
    camera: Camera,

    // is the simulation running?
    paused: bool,

    // Currently grabbed object with the mouse
    grabbed_object: Option<RigidBodyHandle<f64>>,
    grabbed_object_joint: Option<Rc<RefCell<Fixed<f64>>>>,

    // Mouse position in window space
    mouse_position: na::Vector2<f64>,
    // Mouse position in world space
    mouse_position_world: na::Point2<f64>,
    // First click in window space
    first_click: na::Vector2<f64>,
    // First click in world space
    first_click_world: na::Point2<f64>,

    current_action: Action,
    action_step: i8,

    move_camera_up: bool,
    move_camera_down: bool,
    move_camera_left: bool,
    move_camera_right: bool,

    // Holds constraints to be drawn
    constraints: Vec<Constraint<f64>>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        world.set_gravity(na::Vector2::new(0.0, 30.0));

        // Creates the ground
        let rb = RigidBody::new_static(Plane2::new(na::Vector2::new(0.0, -1.0)), 0.3, 0.6);
        world.add_rigid_body(rb);

        // Creating cuboids for pyramid
        let num = 35;
        let rad = 0.5;
        let shift = 2.5 * rad;
        let centerx = shift * (num as f64) / 2.0;

        for i in 0usize..num {
            for j in i..num {
                let fj = j as f64;
                let fi = i as f64;
                let x = (fi * shift / 2.0) + (fj - fi) * 2.5 * rad - centerx;
                let y = -fi * 2.5 * rad - 0.04 - rad;

                let mut rb = RigidBody::new_dynamic(Cuboid2::new(na::Vector2::new(rad - 0.04,
                                                                                  rad - 0.04)),
                                                    1.0,
                                                    0.3,
                                                    0.6);
                rb.append_translation(&na::Translation2::new(x, y));
                world.add_rigid_body(rb);
            }
        }

        Game {
            world: world,
            camera: Camera::new(800, 600),

            paused: true,

            grabbed_object: None,
            grabbed_object_joint: None,

            mouse_position: na::zero(),
            mouse_position_world: na::Point2::new(0.0, 0.0),
            first_click: na::zero(),
            first_click_world: na::Point2::new(0.0, 0.0),

            current_action: Action::None,
            action_step: 0,

            move_camera_up: false,
            move_camera_down: false,
            move_camera_left: false,
            move_camera_right: false,

            constraints: vec![],
        }
    }

    fn trans_camera(&mut self, dt: f64) {
        let camera_move_speed = 100.0;

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

        delta *= dt;
        self.camera.trans(&delta);
    }

    fn get_body_at_mouse(&self) -> Option<RigidBodyHandle<f64>> {
        for b in
            self.world
                .collision_world()
                .interferences_with_point(&self.mouse_position_world, &CollisionGroups::new()) {
            if let WorldObject::RigidBody(ref rb) = b.data {
                return Some(rb.clone());
            }
        }

        None
    }

    fn zoom_in(&mut self) {
        let zoom = self.camera.zoom() * 4.0 / 3.0;
        let zoom = if zoom > MAX_ZOOM { MAX_ZOOM } else { zoom };
        self.camera.set_zoom(zoom);
    }

    fn zoom_out(&mut self) {
        let zoom = self.camera.zoom() * 3.0 / 4.0;
        let zoom = if zoom < MIN_ZOOM { MIN_ZOOM } else { zoom };
        self.camera.set_zoom(zoom);
    }
}

impl State for Game {
    fn update(&mut self, dt: f64) {
        // Constant timestep for physics is important
        let timestep = 1.0 / 60.0;
        if !self.paused {
            self.world.step(timestep);

            self.constraints.clear();
            self.world.constraints(&mut self.constraints);
        }

        self.trans_camera(dt);
    }

    fn render(&self, c: &Context, g: &mut GlGraphics) {
        for rb in self.world.rigid_bodies() {
            let object = WorldObject::RigidBody(rb.clone());
            let bobject = object.borrow();
            let transform = bobject.position();
            let position = self.camera.from_local(&transform.translation.vector);
            let rotation = transform.rotation.angle();
            let shape = bobject.shape().as_ref();
            let margin = bobject.margin();

            let c = c.trans(position.x, position.y)
                .rot_rad(rotation)
                .zoom(self.camera.zoom());

            if let Some(s) = shape.as_shape::<Ball2<f64>>() {
                let radius = s.radius() + margin;
                view::draw_ball(radius, [1.0; 4], &c, g);
            } else if let Some(s) = shape.as_shape::<Cuboid2<f64>>() {
                let width = s.half_extents().x + margin;
                let height = s.half_extents().y + margin;
                view::draw_cuboid(width, height, [1.0; 4], &c, g);
            }
        }

        for constraint in &self.constraints {
            match *constraint {
                Constraint::RBRB(_, _, ref contact) => {
                    let world1 = contact.world1;
                    let world1 = self.camera
                        .from_local(&na::Vector2::new(world1.x, world1.y));
                    let world2 = contact.world2;
                    let world2 = self.camera
                        .from_local(&na::Vector2::new(world2.x, world2.y));
                    graphics::Line::new([0.0, 1.0, 0.0, 1.0], 3.0)
                        .draw([world1.x, world1.y, world2.x, world2.y],
                              &c.draw_state,
                              c.transform,
                              g);

                    let center = na::center(&na::Point2::new(world1.x, world1.y),
                                            &na::Point2::new(world2.x, world2.y));
                    let center_normal_depth = center + contact.normal * contact.depth;
                    graphics::Line::new([0.0, 1.0, 0.0, 1.0], 3.0).draw([center.x,
                                                                         center.y,
                                                                         center_normal_depth.x,
                                                                         center_normal_depth.y],
                                                                        &c.draw_state,
                                                                        c.transform,
                                                                        g);

                    let center_normal = center + contact.normal;
                    graphics::Line::new([0.0, 1.0, 0.0, 1.0], 3.0)
                        .draw([center.x, center.y, center_normal.x, center_normal.y],
                              &c.draw_state,
                              c.transform,
                              g);
                }

                Constraint::BallInSocket(ref bis) => {
                    let anchor1_pos = bis.borrow().anchor1_pos();
                    let anchor1_pos =
                        self.camera
                            .from_local(&na::Vector2::new(anchor1_pos.x, anchor1_pos.y));
                    let anchor2_pos = bis.borrow().anchor2_pos();
                    let anchor2_pos =
                        self.camera
                            .from_local(&na::Vector2::new(anchor2_pos.x, anchor2_pos.y));

                    graphics::Line::new([0.0, 0.0, 1.0, 1.0], 3.0)
                        .draw([anchor1_pos.x, anchor1_pos.y, anchor2_pos.x, anchor2_pos.y],
                              &c.draw_state,
                              c.transform,
                              g);
                }

                Constraint::Fixed(ref f) => {
                    let anchor1_pos = f.borrow().anchor1_pos().translation.vector;
                    let anchor1_pos =
                        self.camera
                            .from_local(&na::Vector2::new(anchor1_pos.x, anchor1_pos.y));
                    let anchor2_pos = f.borrow().anchor2_pos().translation.vector;
                    let anchor2_pos =
                        self.camera
                            .from_local(&na::Vector2::new(anchor2_pos.x, anchor2_pos.y));

                    graphics::Line::new([1.0, 0.0, 0.0, 1.0], 3.0)
                        .draw([anchor1_pos.x, anchor1_pos.y, anchor2_pos.x, anchor2_pos.y],
                              &c.draw_state,
                              c.transform,
                              g);

                }
            }
        }

        if self.current_action != Action::None {
            match self.current_action {
                Action::CreatingBall if self.action_step == 1 => {
                    let radius = na::distance(&self.first_click_world, &self.mouse_position_world);
                    let radius = na::clamp(radius, MIN_BALL_RADIUS, MAX_BALL_RADIUS);
                    let dradius = radius * 2.0;

                    graphics::Ellipse::new(color::WHITE)
                        .resolution(16)
                        .draw([-radius, -radius, dradius, dradius],
                              &c.draw_state,
                              c.trans(self.first_click.x, self.first_click.y)
                                  .zoom(self.camera.zoom())
                                  .transform,
                              g);
                }

                Action::CreatingCuboid if self.action_step == 1 => {
                    let width = self.mouse_position_world.x - self.first_click_world.x;
                    let width = if na::abs(&width) < MIN_CUBOID_WIDTH {
                        if width < 0.0 {
                            -MIN_CUBOID_WIDTH
                        } else {
                            MIN_CUBOID_WIDTH
                        }
                    } else if na::abs(&width) > MAX_CUBOID_WIDTH {
                        if width < 0.0 {
                            -MAX_CUBOID_WIDTH
                        } else {
                            MAX_CUBOID_WIDTH
                        }
                    } else {
                        width
                    };
                    let dwidth = width * 2.0;

                    let height = self.mouse_position_world.y - self.first_click_world.y;
                    let height = if na::abs(&height) < MIN_CUBOID_HEIGHT {
                        if height < 0.0 {
                            -MIN_CUBOID_HEIGHT
                        } else {
                            MIN_CUBOID_HEIGHT
                        }
                    } else if na::abs(&height) > MAX_CUBOID_HEIGHT {
                        if height < 0.0 {
                            -MAX_CUBOID_HEIGHT
                        } else {
                            MAX_CUBOID_HEIGHT
                        }
                    } else {
                        height
                    };
                    let dheight = height * 2.0;


                    graphics::Rectangle::new(color::WHITE).draw([-width, -height, dwidth, dheight],
                                                                &c.draw_state,
                                                                c.trans(self.first_click.x,
                                                                        self.first_click.y)
                                                                    .zoom(self.camera.zoom())
                                                                    .transform,
                                                                g);
                }

                Action::CreatingBallInSocket => {
                    let radius = 5.0;
                    let dradius = radius * 2.0;

                    graphics::Ellipse::new_border([0.0, 0.0, 1.0, 1.0], 0.3)
                        .resolution(16)
                        .draw([-radius, -radius, dradius, dradius],
                              &c.draw_state,
                              c.trans(self.mouse_position.x, self.mouse_position.y)
                                  .transform,
                              g);

                    let radius = 3.0;
                    let dradius = radius * 2.0;

                    graphics::Ellipse::new_border([0.0, 0.0, 1.0, 1.0], 0.3)
                        .resolution(16)
                        .draw([-radius, -radius, dradius, dradius],
                              &c.draw_state,
                              c.trans(self.mouse_position.x, self.mouse_position.y)
                                  .transform,
                              g);
                }

                // All other actions
                _ => {}
            }
        }
    }

    fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
        let mapped_coords = self.camera.to_local(&self.mouse_position);
        self.mouse_position_world.x = mapped_coords.x;
        self.mouse_position_world.y = mapped_coords.y;

        let attach2 = na::Isometry2::new(self.mouse_position_world.coords, 0.0);
        if self.grabbed_object.is_some() {
            let joint = self.grabbed_object_joint.as_ref().unwrap();
            joint.borrow_mut().set_local2(attach2);
        }
    }

    fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if self.current_action == Action::None {
                if pressed {
                    self.grabbed_object = self.get_body_at_mouse();

                    if let Some(ref b) = self.grabbed_object {
                        if let Some(ref j) = self.grabbed_object_joint {
                            self.world.remove_fixed(j);
                        }

                        let attach2 = na::Isometry2::new(self.mouse_position_world.coords, 0.0);
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
                    self.first_click_world = self.mouse_position_world;
                    self.action_step += 1;
                } else if !pressed && self.action_step == 1 {
                    let radius = na::distance(&self.first_click_world, &self.mouse_position_world);
                    let radius = na::clamp(radius, MIN_BALL_RADIUS, MAX_BALL_RADIUS);
                    if radius > 0.0 {
                        self.current_action = Action::None;

                        let ball = Ball2::new(radius);
                        let mut rb = RigidBody::new_dynamic(ball, 1.0, 0.3, 0.6);
                        rb.append_translation(&na::Translation2::new(self.first_click_world.x,
                                                                     self.first_click_world.y));
                        self.world.add_rigid_body(rb);
                    }
                }
            } else if self.current_action == Action::CreatingCuboid {
                if pressed && self.action_step == 0 {
                    self.first_click = self.mouse_position;
                    self.first_click_world = self.mouse_position_world;
                    self.action_step += 1;
                } else if !pressed && self.action_step == 1 &&
                          self.mouse_position != self.first_click {
                    self.current_action = Action::None;

                    let width = self.mouse_position_world.x - self.first_click_world.x;
                    let width = na::abs(&width);
                    let width = na::clamp(width, MIN_CUBOID_WIDTH, MAX_CUBOID_WIDTH);

                    let height = self.mouse_position_world.y - self.first_click_world.y;
                    let height = na::abs(&height);
                    let height = na::clamp(height, MIN_CUBOID_HEIGHT, MAX_CUBOID_HEIGHT);

                    let cuboid = Cuboid2::new(na::Vector2::new(width, height));
                    let mut rb = RigidBody::new_dynamic(cuboid, 1.0, 0.3, 0.6);
                    rb.append_translation(&na::Translation2::new(self.first_click_world.x,
                                                                 self.first_click_world.y));
                    self.world.add_rigid_body(rb);
                }
            }
        }
    }

    fn handle_mouse_scroll(&mut self, _: f64, y: f64) {
        if y < 0.0 {
            // Scrolling down
            self.zoom_out();
        } else {
            // Scrolling up
            self.zoom_in();
        }
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

            Key::D4 if pressed => {
                self.current_action = Action::CreatingBallInSocket;
                self.action_step = 0;
            }

            Key::Space if pressed => {
                self.paused = !self.paused;
            }

            Key::W if pressed => self.zoom_in(),
            Key::S if pressed => self.zoom_out(),

            _ => (),
        }
    }

    fn handle_resize(&mut self, width: u32, height: u32) {
        self.camera.set_size(width, height);
    }
}
