use piston_window::{Context, G2d, Rectangle, Transformed};

use nphysics2d::object::RigidBodyHandle;

use camera::Camera;

pub struct Cuboid {
    body: RigidBodyHandle<f64>,
    width: f64,
    height: f64,
    gfx: Rectangle,
}

impl Cuboid {
    pub fn new(width: f64, height: f64, color: [f32; 4], body: RigidBodyHandle<f64>) -> Self {
        let margin = body.borrow().margin();
        Cuboid {
            body: body,
            width: width + margin,
            height: height + margin,
            gfx: Rectangle::new(color),
        }
    }

    pub fn render(&self, camera: &Camera, c: &Context, g: &mut G2d) {
        let body = self.body.borrow();
        let position = body.position();
        let rotation = position.rotation.angle();
        let position = camera.coord_to_window(position.translation.vector);

        self.gfx
            .draw([-self.width,
                   -self.height,
                   self.width * 2.0,
                   self.height * 2.0],
                  &c.draw_state,
                  c.trans(position.x, position.y)
                      .rot_rad(rotation)
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }
}
