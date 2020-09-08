#![allow(dead_code)]

use crate::vec3;

struct Ray {
    orig: vec3::Point3,
    dir: vec3::Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> vec3::Point3 {
        self.orig + self.dir * t
    }

    /// returns hit point if hit
    pub fn hit_sphere(
        &self,
        center: vec3::Point3,
        radius: f32,
    ) -> Option<vec3::Point3> {
        let r = &self;
        let oc = r.orig - center;
        let a = r.dir.dot(r.dir);
        let b = 2_f32 * oc.dot(r.dir);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4_f32 * a * c;

        if discriminant < 0_f32 {
            return None;
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 {
                return Some(r.at(t));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::image_writer::*;
    use crate::ray::Ray;
    use crate::vec3::*;

    fn ray_color(r: &Ray) -> Color {
        let center = Vec3::new(0_f32, 0_f32, -1_f32);
        let radius = 0.5_f32;
        match r.hit_sphere(center, radius) {
            Some(p) => {
                let normal = (p - center).unit_vector();
                return Color::new(
                    normal.x() + 1.0,
                    normal.y() + 1.0,
                    normal.z() + 1.0,
                ) * 0.5;
            }
            None => {}
        }

        let unit_direction = r.dir.unit_vector();
        let t: f32 = 0.5_f32 * (unit_direction.y() + 1_f32);

        let sky_blue = Vec3(0.5_f32, 0.7_f32, 1.0_f32);
        let white = Vec3(1_f32, 1_f32, 1_f32);
        Color::lerp(white, sky_blue, t)
    }

    #[test]
    fn test() {
        #![allow(non_upper_case_globals)]

        // Image
        const aspect_ratio: f32 = 16_f32 / 9_f32;
        const image_width: u32 = 400;
        const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

        // Camera
        const viewport_height: f32 = 2.0;
        const viewport_width: f32 = viewport_height * aspect_ratio;
        const focal_length: f32 = 1.0;

        const origin: Point3 = Vec3(0_f32, 0_f32, 0_f32);
        const horizontal: Vec3 = Vec3(viewport_width, 0_f32, 0_f32);
        const vertical: Vec3 = Vec3(0_f32, viewport_height, 0_f32);
        let lower_left_corner: Point3 = origin
            - horizontal * 0.5f32
            - vertical * 0.5_f32
            - Vec3(0_f32, 0_f32, focal_length);

        // Render
        let mut col_vec = Vec::<Pixel>::new();
        for j in (0..image_height).rev() {
            println!("Scanlines remaining: {}", j);
            for i in 0..image_width {
                let u = i as f32 / (image_width - 1) as f32;
                let v = j as f32 / (image_height - 1) as f32;
                let r = Ray {
                    orig: origin,
                    dir: lower_left_corner + horizontal * u + vertical * v
                        - origin,
                };
                let color = ray_color(&r);
                col_vec.push(Pixel { color, x: i, y: j });
            }
        }

        let res = crate::image_writer::write_ppm(
            image_width,
            image_height,
            &col_vec,
            std::path::Path::new("images/blue_sky.ppm"),
        );
        assert!(res.is_ok());

        let res = crate::image_writer::write_png(
            image_width,
            image_height,
            &col_vec,
            std::path::Path::new("images/blue_sky.png"),
        );
        assert!(res.is_ok());

        println!("Done!")
    }
}
