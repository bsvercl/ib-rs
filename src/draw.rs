use piston_window::{self, Context, G2d, Transformed};

use camera::Camera;
use object::{Ball, Cuboid};

pub struct Draw {}

impl Draw {
    pub fn new() -> Self {
        Draw {}
    }

    pub fn render_ball(&self, ball: &Ball, camera: &Camera, c: &Context, g: &mut G2d) {
        let bobject = ball.object.borrow();
        let transform = bobject.position();
        let position = camera.coord_to_window(transform.translation.vector);
        let radius = ball.radius;
        let dradius = radius * 2.0;

        piston_window::Ellipse::new(ball.color)
            .resolution(16)
            .draw([-radius, -radius, dradius, dradius],
                  &c.draw_state,
                  c.trans(position.x, position.y)
                      .rot_rad(transform.rotation.angle())
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }

    pub fn render_cuboid(&self, cuboid: &Cuboid, camera: &Camera, c: &Context, g: &mut G2d) {
        let bobject = cuboid.object.borrow();
        let transform = bobject.position();
        let position = camera.coord_to_window(transform.translation.vector);
        let width = cuboid.width;
        let dwidth = width * 2.0;
        let height = cuboid.height;
        let dheight = height * 2.0;

        piston_window::Rectangle::new(cuboid.color).draw([-width, -height, dwidth, dheight],
                                                         &c.draw_state,
                                                         c.trans(position.x, position.y)
                                                             .rot_rad(transform
                                                                          .rotation
                                                                          .angle())
                                                             .zoom(camera.zoom())
                                                             .transform,
                                                         g);
    }
}
