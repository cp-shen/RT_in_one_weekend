#![allow(dead_code)]

use crate::vec3;

struct Ray {
    orig: vec3::Point3,
    dir: vec3::Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> vec3::Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vec3::Color;
    use crate::vec3::Point3;
    use crate::vec3::Vec3;

    fn ray_color(r: &Ray) -> Color {
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
        const viewport_width: f32 = viewport_height / aspect_ratio;
        const focal_length: f32 = 1.0;

        const origin: Point3 = Vec3(0_f32, 0_f32, 0_f32);
        const horizontal: Vec3 = Vec3(viewport_width, 0_f32, 0_f32);
        const vertical: Vec3 = Vec3(0_f32, viewport_height, 0_f32);
        let lower_left_corner: Point3 = origin
            - horizontal * 0.5f32
            - vertical * 0.5_f32
            - Vec3(0_f32, 0_f32, focal_length);

        // Render
        let mut col_vec = Vec::<u8>::new();
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
                col_vec.append(&mut color.float_color_to_8bit());
            }
        }
        let res = crate::image_writer::write_ppm(
            image_width as usize,
            image_height as usize,
            &col_vec,
            std::path::Path::new("images/blue_sky.ppm"),
        );
        assert!(res.is_ok());

        let res = crate::image_writer::write_png(
            image_width as usize,
            image_height as usize,
            &col_vec,
            std::path::Path::new("images/blue_sky.png"),
        );
        assert!(res.is_ok());

        println!("Done!")
    }
}
