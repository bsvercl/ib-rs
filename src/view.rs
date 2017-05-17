use color;
use graphics::{self, Colored, Context};
use opengl_graphics::GlGraphics;

pub fn draw_ball(radius: f64, c: &Context, g: &mut GlGraphics) {
    let dradius = radius * 2.0;

    graphics::Ellipse::new(color::WHITE)
        .border(graphics::ellipse::Border {
                    color: color::WHITE.shade(0.5),
                    radius: 0.1,
                })
        .resolution(16)
        .draw([-radius, -radius, dradius, dradius],
              &c.draw_state,
              c.transform,
              g);
}

pub fn draw_cuboid(width: f64, height: f64, c: &Context, g: &mut GlGraphics) {
    let dwidth = width * 2.0;
    let dheight = height * 2.0;

    graphics::Rectangle::new(color::WHITE).draw([-width, -height, dwidth, dheight],
                                                &c.draw_state,
                                                c.transform,
                                                g);
}
