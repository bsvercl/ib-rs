use na;

pub struct Camera {
    position: na::Vector2<f64>,
    zoom: f64,
    size: na::Vector2<f64>,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Camera {
            position: na::zero(),
            zoom: 30.0,
            size: na::Vector2::new(width as f64, height as f64),
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size.x = width as f64;
        self.size.y = height as f64;
    }

    pub fn trans(&mut self, xy: &na::Vector2<f64>) {
        self.position += xy;
    }
}

impl Camera {
    pub fn to_local(&self, xy: &na::Vector2<f64>) -> na::Vector2<f64> {
        self.position + (xy - self.size / 2.0) / self.zoom
    }

    pub fn from_local(&self, xy: &na::Vector2<f64>) -> na::Vector2<f64> {
        self.zoom * (xy - self.position) + self.size / 2.0
    }
}
