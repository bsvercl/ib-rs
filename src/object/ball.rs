use piston_window::{Context, G2d, Ellipse, Transformed};

use nphysics2d::object::RigidBodyHandle;

use camera::Camera;

pub struct Ball {
    body: RigidBodyHandle<f64>,
    radius: f64,
    gfx: Ellipse,
}

impl Ball {
    pub fn new(radius: f64, color: [f32; 4], body: RigidBodyHandle<f64>) -> Self {
        let margin = body.borrow().margin();
        Ball {
            body: body,
            radius: radius + margin,
            gfx: Ellipse::new(color),
        }
    }

    pub fn render(&self, camera: &Camera, c: &Context, g: &mut G2d) {
        let body = self.body.borrow();
        let position = body.position();
        let rotation = position.rotation.angle();
        let position = camera.coord_to_window(position.translation.vector);

        self.gfx
            .draw([-self.radius,
                   -self.radius,
                   self.radius * 2.0,
                   self.radius * 2.0],
                  &c.draw_state,
                  c.trans(position.x, position.y)
                      .rot_rad(rotation)
                      .zoom(camera.zoom())
                      .transform,
                  g);
    }
}
