use na;

pub struct Camera {
    center: na::Vector2<f64>,

    pub zoom: f64,

    width: f64,
    height: f64,
}

impl Camera {
    pub fn new(width: f64, height: f64, zoom: f64) -> Self {
        Camera {
            center: na::zero(),

            zoom: zoom,

            width: width,
            height: height,
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    #[allow(dead_code)]
    pub fn set_center(&mut self, center: na::Vector2<f64>) {
        self.center = center;
    }

    #[allow(dead_code)]
    pub fn center(&self) -> na::Vector2<f64> {
        self.center
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn trans(&mut self, xy: na::Vector2<f64>) {
        self.center += xy;
    }
}

impl Camera {
    pub fn window_to_coord(&self, xy: na::Vector2<f64>) -> na::Vector2<f64> {
        na::Vector2::new(self.center.x + (xy.x - self.width / 2.0) / self.zoom,
                         self.center.y + (xy.y - self.height / 2.0) / self.zoom)
    }

    pub fn coord_to_window(&self, xy: na::Vector2<f64>) -> na::Vector2<f64> {
        na::Vector2::new(self.zoom * (xy.x - self.center.x) + (self.width / 2.0),
                         self.zoom * (xy.y - self.center.y) + (self.height / 2.0))
    }
}
