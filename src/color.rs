use piston_window::types::{Color, ColorComponent};

macro_rules! make_color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        [$r as ColorComponent / 255.0,
         $g as ColorComponent / 255.0,
         $b as ColorComponent / 255.0,
         $a as ColorComponent / 255.0]
    );

    ($r:expr, $g:expr, $b:expr) => (
        make_color!($r, $g, $b, 255);
    );
}

pub const CORNFLOWER_BLUE: Color = make_color!(100, 149, 237);
#[allow(dead_code)]
pub const BLACK: Color = make_color!(0, 0, 0);
pub const WHITE: Color = make_color!(255, 255, 255);
