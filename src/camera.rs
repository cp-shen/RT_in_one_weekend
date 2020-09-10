use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    pub orig: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        #![allow(non_upper_case_globals)]

        const aspect_ratio: f32 = 16_f32 / 9_f32;
        const viewport_height: f32 = 2.0;
        const viewport_width: f32 = viewport_height * aspect_ratio;
        const focal_length: f32 = 1.0;

        const orig: Point3 = Vec3(0_f32, 0_f32, 0_f32);
        const horizontal: Vec3 = Vec3(viewport_width, 0_f32, 0_f32);
        const vertical: Vec3 = Vec3(0_f32, viewport_height, 0_f32);
        let lower_left_corner: Point3 = orig
            - horizontal * 0.5f32
            - vertical * 0.5_f32
            - Vec3(0_f32, 0_f32, focal_length);

        Camera {
            orig,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        let orig = self.orig;
        let dir =
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - orig;
        Ray { orig, dir }
    }
}
