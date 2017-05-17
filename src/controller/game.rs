use super::Controller;
use camera::Camera;
use color;
use graphics::{self, Context, Transformed};
use na;
use ncollide;
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
    camera: Camera,

    paused: bool,

    grabbed_object: Option<RigidBodyHandle<f64>>,
    grabbed_object_joint: Option<Rc<RefCell<Fixed<f64>>>>,
    mouse_position: na::Vector2<f64>,

    current_action: Action,
    action_step: i8,
    first_click: na::Vector2<f64>,

    move_camera_up: bool,
    move_camera_down: bool,
    move_camera_left: bool,
    move_camera_right: bool,

    collisions: Vec<Constraint<f64>>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        world.set_gravity(na::Vector2::new(0.0, 30.0));
        Game {
            world: world,
            camera: Camera::new(800, 600),

            paused: true,

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

            collisions: vec![],
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

        self.camera.trans(delta * dt);
    }

    fn get_body_at_mouse(&self) -> Option<RigidBodyHandle<f64>> {
        let mapped_coords = self.camera.to_local(self.mouse_position);
        let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);

        for b in self.world
                .collision_world()
                .interferences_with_point(&mapped_point, &CollisionGroups::new()) {
            if let WorldObject::RigidBody(ref rb) = b.data {
                return Some(rb.clone());
            }
        }

        None
    }
}

impl Controller for Game {
    fn update(&mut self, dt: f64) {
        let timestep = 1.0 / 60.0;
        if !self.paused {
            self.world.step(timestep);
            self.collisions.clear();
            self.world.constraints(&mut self.collisions);
        }

        self.trans_camera(dt);
    }

    fn render(&self, c: &Context, g: &mut GlGraphics) {
        for rb in self.world.rigid_bodies() {
            let object = WorldObject::RigidBody(rb.clone());
            let bobject = object.borrow();
            let transform = bobject.position();
            let position = self.camera.from_local(transform.translation.vector);
            let rotation = transform.rotation.angle();
            let shape = bobject.shape().as_ref();
            let margin = bobject.margin();

            let c = c.trans(position.x, position.y)
                .rot_rad(rotation)
                .zoom(self.camera.zoom());

            if let Some(s) = shape.as_shape::<ncollide::shape::Ball2<f64>>() {
                let radius = s.radius() + margin;
                view::draw_ball(radius, &c, g);
            } else if let Some(s) = shape.as_shape::<ncollide::shape::Cuboid2<f64>>() {
                let width = s.half_extents().x + margin;
                let height = s.half_extents().y + margin;
                view::draw_cuboid(width, height, &c, g);
            }
        }

        for collision in &self.collisions {
            match *collision {
                Constraint::RBRB(_, _, ref contact) => {
                    let world1 = contact.world1;
                    let world1 = self.camera.from_local(na::Vector2::new(world1.x, world1.y));
                    let world2 = contact.world2;
                    let world2 = self.camera.from_local(na::Vector2::new(world2.x, world2.y));
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
                            .from_local(na::Vector2::new(anchor1_pos.x, anchor1_pos.y));
                    let anchor2_pos = bis.borrow().anchor2_pos();
                    let anchor2_pos =
                        self.camera
                            .from_local(na::Vector2::new(anchor2_pos.x, anchor2_pos.y));

                    graphics::Line::new([0.0, 0.0, 1.0, 1.0], 3.0)
                        .draw([anchor1_pos.x, anchor1_pos.y, anchor2_pos.x, anchor2_pos.y],
                              &c.draw_state,
                              c.transform,
                              g);
                }

                _ => {}
            }
        }

        if self.current_action != Action::None {
            let mapped_first_click = na::Point2::new(self.first_click.x, self.first_click.y);
            let mapped_mouse_position = na::Point2::new(self.mouse_position.x,
                                                        self.mouse_position.y);

            match self.current_action {
                Action::CreatingBall if self.action_step == 1 => {
                    let radius = na::distance(&mapped_first_click, &mapped_mouse_position);
                    let radius = if radius < 0.1 {
                        0.1
                    } else if radius > 10.0 {
                        10.0
                    } else {
                        radius
                    };
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
                    let width = mapped_mouse_position.x - mapped_first_click.x;
                    let width = if na::abs(&width) < 0.1 {
                        if width < 0.0 { -0.1 } else { 0.1 }
                    } else if na::abs(&width) > 10.0 {
                        if width < 0.0 { -10.0 } else { 10.0 }
                    } else {
                        width
                    };
                    let dwidth = width * 2.0;

                    let height = mapped_mouse_position.y - mapped_first_click.y;
                    let height = if na::abs(&width) < 0.1 {
                        if height < 0.0 { -0.1 } else { 0.1 }
                    } else if na::abs(&height) > 10.0 {
                        if height < 0.0 { -10.0 } else { 10.0 }
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
                              c.trans(mapped_mouse_position.x, mapped_mouse_position.y)
                                  .transform,
                              g);

                    let radius = 3.0;
                    let dradius = radius * 2.0;

                    graphics::Ellipse::new_border([0.0, 0.0, 1.0, 1.0], 0.3)
                        .resolution(16)
                        .draw([-radius, -radius, dradius, dradius],
                              &c.draw_state,
                              c.trans(mapped_mouse_position.x, mapped_mouse_position.y)
                                  .transform,
                              g);
                }
                _ => {}
            }
        }
    }

    fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_position = na::Vector2::new(x, y);

        let mapped_coords = self.camera.to_local(self.mouse_position);
        let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);
        let attach2 = na::Isometry2::new(mapped_point.coords, 0.0);
        if self.grabbed_object.is_some() {
            let joint = self.grabbed_object_joint.as_ref().unwrap();
            joint.borrow_mut().set_local2(attach2);
        }
    }

    fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if self.current_action == Action::None {
                if pressed {
                    let mapped_coords = self.camera.to_local(self.mouse_position);
                    let mapped_point = na::Point2::new(mapped_coords.x, mapped_coords.y);

                    self.grabbed_object = self.get_body_at_mouse();

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
                    let first_click = self.camera.to_local(self.first_click);
                    let mapped_first_click = na::Point2::new(first_click.x, first_click.y);
                    let mouse_position = self.camera.to_local(self.mouse_position);
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

                        let pos = self.camera.to_local(self.first_click);

                        let ball = ncollide::shape::Ball2::new(radius);
                        let mut rb = RigidBody::new_dynamic(ball, 1.0, 0.3, 0.6);
                        rb.append_translation(&na::Translation2::new(pos.x, pos.y));
                        self.world.add_rigid_body(rb);
                    }
                }
            } else if self.current_action == Action::CreatingCuboid {
                if pressed && self.action_step == 0 {
                    self.first_click = self.mouse_position;
                    self.action_step += 1;
                } else if !pressed && self.action_step == 1 &&
                          self.mouse_position != self.first_click {
                    self.current_action = Action::None;

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

                    let pos = self.camera.to_local(self.first_click);

                    let cuboid = ncollide::shape::Cuboid2::new(na::Vector2::new(width, height));
                    let mut rb = RigidBody::new_dynamic(cuboid, 1.0, 0.3, 0.6);
                    rb.append_translation(&na::Translation2::new(pos.x, pos.y));
                    self.world.add_rigid_body(rb);
                }
            }
        }
    }

    fn handle_mouse_scroll(&mut self, _: f64, y: f64) {
        if y < 0.0 {
            let zoom = self.camera.zoom() * 3.0 / 4.0;
            let zoom = if zoom < 12.0 { 12.0 } else { zoom };
            self.camera.set_zoom(zoom);
        } else {
            let zoom = self.camera.zoom() * 4.0 / 3.0;
            let zoom = if zoom > 75.0 { 75.0 } else { zoom };
            self.camera.set_zoom(zoom);
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

            Key::W if pressed => {
                let zoom = self.camera.zoom() * 4.0 / 3.0;
                let zoom = if zoom > 75.0 { 75.0 } else { zoom };
                self.camera.set_zoom(zoom);
            }

            Key::S if pressed => {
                let zoom = self.camera.zoom() * 3.0 / 4.0;
                let zoom = if zoom < 12.0 { 12.0 } else { zoom };
                self.camera.set_zoom(zoom);
            }

            _ => (),
        }
    }

    fn handle_resize(&mut self, width: u32, height: u32) {
        self.camera.set_size(width, height);
    }
}
