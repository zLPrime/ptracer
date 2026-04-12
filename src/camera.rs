use rayon::iter::ParallelIterator;

use crate::primitives::{Color, rand};

use super::scene::get_ray_color;

use super::{
    canvas::Canvas,
    vec3d::{Point3d, Vec3d},
    Ray, Scene,
};

#[derive(Debug)]
pub struct Camera {
    pub location: Point3d,
    pub direction: Vec3d,
}

impl Camera {
    pub fn render(&self, canvas: &mut Canvas, scene: &Scene) {
        let width = canvas.width;
        let height = canvas.height;
        let ratio = width as f32 / height as f32;
        let left = self.get_left().normalize() * ratio;
        let top = self.direction.cross(&left).normalize();
        let top_left = self.direction + top + left;
        let step_v = 2. / (width - 1) as f32;
        let step_h = 2. / (height - 1) as f32;
        let half_step_v = step_v / 2.;
        let half_step_h = step_h / 2.;

        canvas.par_rows_mut().for_each(|mut row| {
            let y = row.y;
            let current_v = top_left - (top * (step_h * (y) as f32));
            for x in 0..width {
                let mut color = Color::new(0., 0., 0.);
                let factor = 1;
                for _ in 0..factor {
                    let h_deviation = rand::get_random_float_neg_pos(half_step_h);
                    let v_deviation = rand::get_random_float_neg_pos(half_step_v);
                    let direction = current_v - left * (step_v * (x) as f32 + v_deviation) + top * h_deviation;
                    let ray = Ray {
                        origin: self.location,
                        direction,
                    };
                    let ray_color = get_ray_color(&ray, &scene, 2);
                    color = color + ray_color * (1. / factor as f32)
                }
                row.draw_pixel(x, color);
            }
        })
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
