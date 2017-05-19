use piston_window::{self, Context, Colored, G2d};
use piston_window::types::Color;

// TODO: Use `Matrix2d` instead of `Context`?
pub fn draw_ball(radius: f64, color: Color, c: &Context, g: &mut G2d) {
    let dradius = radius * 2.0;

    piston_window::Ellipse::new(color)
        .border(piston_window::ellipse::Border {
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
pub fn draw_cuboid(width: f64, height: f64, color: Color, c: &Context, g: &mut G2d) {
    let dwidth = width * 2.0;
    let dheight = height * 2.0;

    piston_window::Rectangle::new(color)
        .border(piston_window::rectangle::Border {
                    color: color.shade(0.5),
                    radius: 0.1,
                })
        .draw([-width, -height, dwidth, dheight],
              &c.draw_state,
              c.transform,
              g);
}
