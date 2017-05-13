use graphics::{self, Colored, Context, Transformed};
use opengl_graphics::GlGraphics;

use na;

use camera::Camera;
use object::{Ball, Cuboid};

pub struct Draw {}

impl Draw {
    pub fn new() -> Self {
        Draw {}
    }

    pub fn render_temp_ball_in_socket(&self,
                                      mouse_posiiton: na::Vector2<f64>,
                                      c: &Context,
                                      g: &mut GlGraphics) {
        let mapped_mouse_position = na::Point2::new(mouse_posiiton.x, mouse_posiiton.y);

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

    pub fn render_temp_ball(&self,
                            first_click: na::Vector2<f64>,
                            mouse_position: na::Vector2<f64>,
                            camera: &Camera,
                            c: &Context,
                            g: &mut GlGraphics) {
        let mapped_first_click = na::Point2::new(first_click.x, first_click.y);
        let mapped_mouse_position = na::Point2::new(mouse_position.x, mouse_position.y);
        let radius = na::distance(&mapped_first_click, &mapped_mouse_position);
        let radius = if radius < 0.1 {
            0.1
        } else if radius > 10.0 {
            10.0
        } else {
            radius
        };
        let dradius = radius * 2.0;

        graphics::Ellipse::new([1.0; 4])
            .resolution(16)
            .draw([-radius, -radius, dradius, dradius],
                  &c.draw_state,
                  c.trans(first_click.x, first_click.y)
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }

    pub fn render_temp_cuboid(&self,
                              first_click: na::Vector2<f64>,
                              mouse_position: na::Vector2<f64>,
                              camera: &Camera,
                              c: &Context,
                              g: &mut GlGraphics) {
        let mapped_first_click = na::Point2::new(first_click.x, first_click.y);
        let mapped_mouse_position = na::Point2::new(mouse_position.x, mouse_position.y);

        let width = mapped_mouse_position.x - mapped_first_click.x;
        let width = if na::abs(&width) < 0.1 {
            if width < 0.0 { -0.1 } else { 0.1 }
        } else if na::abs(&width) > 10.0 {
            if width < 0.0 { -10.0 } else { 10.0 }
        } else {
            width
        };
        let height = mapped_mouse_position.y - mapped_first_click.y;
        let height = if na::abs(&width) < 0.1 {
            if height < 0.0 { -0.1 } else { 0.1 }
        } else if na::abs(&height) > 10.0 {
            if height < 0.0 { -10.0 } else { 10.0 }
        } else {
            height
        };

        let dwidth = width * 2.0;
        let dheight = height * 2.0;

        graphics::Rectangle::new([1.0; 4]).draw([-width, -height, dwidth, dheight],
                                                &c.draw_state,
                                                c.trans(first_click.x, first_click.y)
                                                    .zoom(camera.zoom())
                                                    .transform,
                                                g);
    }

    pub fn render_ball(&self, ball: &Ball, camera: &Camera, c: &Context, g: &mut GlGraphics) {
        let bobject = ball.object.borrow();
        let transform = bobject.position();
        let position = camera.coord_to_window(transform.translation.vector);
        let radius = ball.radius;
        let dradius = radius * 2.0;

        graphics::Ellipse::new(ball.color)
            .border(graphics::ellipse::Border {
                        color: ball.color.shade(0.3),
                        radius: 0.5,
                    })
            .resolution(16)
            .draw([-radius, -radius, dradius, dradius],
                  &c.draw_state,
                  c.trans(position.x, position.y)
                      .rot_rad(transform.rotation.angle())
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }

    pub fn render_cuboid(&self,
                         cuboid: &Cuboid,
                         camera: &Camera,
                         c: &Context,
                         g: &mut GlGraphics) {
        let bobject = cuboid.object.borrow();
        let transform = bobject.position();
        let position = camera.coord_to_window(transform.translation.vector);
        let width = cuboid.width;
        let dwidth = width * 2.0;
        let height = cuboid.height;
        let dheight = height * 2.0;

        graphics::Rectangle::new(cuboid.color)
            .border(graphics::rectangle::Border {
                        color: cuboid.color.shade(0.3),
                        radius: 0.5,
                    })
            .draw([-width, -height, dwidth, dheight],
                  &c.draw_state,
                  c.trans(position.x, position.y)
                      .rot_rad(transform.rotation.angle())
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }
}
