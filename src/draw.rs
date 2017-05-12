use piston_window::{self, Colored, Context, G2d, Transformed};

use camera::Camera;
use object::{Ball, Cuboid};

pub struct Draw {}

impl Draw {
    pub fn new() -> Self {
        Draw {}
    }

    pub fn render_ball(&self, ball: &Ball, camera: &Camera, c: &Context, g: &mut G2d) {
        let body = ball.body.borrow();
        let position = camera.coord_to_window(body.position().translation.vector);
        let radius = ball.radius;
        let dradius = radius * 2.0;

        let color = if !body.is_active() {
            ball.color.shade(0.2)
        } else {
            ball.color
        };

        piston_window::Ellipse::new(color)
            .resolution(16)
            .draw_tri([-radius, -radius, dradius, dradius],
                      &c.draw_state,
                      c.trans(position.x, position.y)
                          .rot_rad(body.position().rotation.angle())
                          .zoom(camera.zoom())
                          .transform,
                      g);

    }

    pub fn render_cuboid(&self, cuboid: &Cuboid, camera: &Camera, c: &Context, g: &mut G2d) {
        let body = cuboid.body.borrow();
        let position = camera.coord_to_window(body.position().translation.vector);
        let width = cuboid.width;
        let dwidth = width * 2.0;
        let height = cuboid.height;
        let dheight = height * 2.0;

        let color = if !body.is_active() {
            cuboid.color.shade(0.5)
        } else {
            cuboid.color
        };

        piston_window::Rectangle::new(color).draw_tri([-width, -height, dwidth, dheight],
                                                      &c.draw_state,
                                                      c.trans(position.x, position.y)
                                                          .rot_rad(body.position()
                                                                       .rotation
                                                                       .angle())
                                                          .zoom(camera.zoom())
                                                          .transform,
                                                      g);

    }
}
