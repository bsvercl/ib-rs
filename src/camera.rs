use na;

pub struct Camera {
    position: na::Vector2<f64>,
    zoom: f64,
    size: na::Vector2<f64>,
}

impl Camera {
    pub fn new(width: f64, height: f64) -> Self {
        Camera {
            position: na::zero(),
            zoom: 30.0,
            size: na::Vector2::new(width, height),
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.size = na::Vector2::new(width, height);
    }

    pub fn trans(&mut self, xy: na::Vector2<f64>) {
        self.position += xy;
    }
}

impl Camera {
    pub fn window_to_coord(&self, xy: na::Vector2<f64>) -> na::Vector2<f64> {
        self.position + (xy - self.size / 2.0) / self.zoom
    }

    pub fn coord_to_window(&self, xy: na::Vector2<f64>) -> na::Vector2<f64> {
        self.zoom * (xy - self.position) + self.size / 2.0
    }
}
