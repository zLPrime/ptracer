use super::scene::get_ray_color;

use super::{
    canvas::Canvas,
    vec3d::{Point3d, Vec3d},
    Ray, Scene,
};

pub struct Camera {
    pub location: Point3d,
    pub direction: Vec3d,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas, scene: &Scene) {
        let ratio = canvas.width as f32 / canvas.height as f32;
        let left = self.get_left().normalize() * ratio;
        let top = self.direction.cross(&left).normalize();
        let top_left = self.direction + top + left;
        let step_v = 2. / (canvas.width - 1) as f32;
        let step_h = 2. / (canvas.height - 1) as f32;
        for y in 0..canvas.height {
            let current_v = top_left - (top * (step_h * (y) as f32));
            for x in 0..canvas.width {
                let direction = current_v - (left * (step_v * (x) as f32));
                let ray = Ray {
                    origin: self.location,
                    direction,
                };
                let ray_color = get_ray_color(&ray, &scene.spheres, 1);
                canvas.draw_pixel(x, y, ray_color);
            }
        }
    }

    pub fn rotate_x(&mut self, theta: f32) {
        self.direction = self.direction.rotate_x(theta)
    }

    pub fn move_forward(&mut self, step: f32) {
        self.location = self.location + self.direction.normalize() * step;
    }

    pub fn move_left(&mut self, step: f32) {
        self.location = self.location + self.get_left().normalize() * step;
    }

    fn get_left(&self) -> Vec3d {
        let top = Vec3d::new(0., 0., 1.);
        top.cross(&self.direction)
    }
}
