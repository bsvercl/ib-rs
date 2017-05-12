use piston_window::types::Color;

macro_rules! make_color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        [$r / 255.0, $g / 255.0, $b / 255.0, $a / 255.0]
    );

    ($r:expr, $g:expr, $b:expr) => (
        make_color!($r, $g, $b, 255.0);
    );
}

pub const CORNFLOWER_BLUE: Color = make_color!(100.0, 149.0, 237.0);
