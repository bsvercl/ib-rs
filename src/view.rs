use graphics::{self, Colored, Context};
use graphics::types::Color;
use opengl_graphics::GlGraphics;

// TODO: Use `Matrix2d` instead of `Context`?
pub fn draw_ball(radius: f64, color: Color, c: &Context, g: &mut GlGraphics) {
    let dradius = radius * 2.0;

    graphics::Ellipse::new(color)
        .border(graphics::ellipse::Border {
                    color: color.shade(0.5),
                    radius: 0.1,
                })
        .resolution(16)
        .draw([-radius, -radius, dradius, dradius],
              &c.draw_state,
              c.transform,
              g);
}

// TODO: Use `Matrix2d` instead of `Context`?
pub fn draw_cuboid(width: f64, height: f64, color: Color, c: &Context, g: &mut GlGraphics) {
    let dwidth = width * 2.0;
    let dheight = height * 2.0;

    graphics::Rectangle::new(color)
        .border(graphics::rectangle::Border {
                    color: color.shade(0.5),
                    radius: 0.1,
                })
        .draw([-width, -height, dwidth, dheight],
              &c.draw_state,
              c.transform,
              g);
}
