#[macro_use]
extern crate derive_new;

mod camera;
mod image_writer;
mod ray;
mod shapes;
mod vec3;

use crate::image_writer::*;
use crate::ray::Ray;
use crate::shapes::*;
use crate::vec3::*;

fn main() {
    draw();
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(r) {
        None => {}
        Some(rec) => return (rec.normal() + Color::new(1.0, 1.0, 1.0)) * 0.5,
    }

    let unit_direction = r.dir().unit_vector();
    let t: f32 = 0.5_f32 * (unit_direction.y() + 1_f32);

    let sky_blue = Vec3(0.5_f32, 0.7_f32, 1.0_f32);
    let white = Vec3(1_f32, 1_f32, 1_f32);
    Color::lerp(white, sky_blue, t)
}

fn draw() {
    #![allow(non_upper_case_globals)]

    // Image
    const aspect_ratio: f32 = 16_f32 / 9_f32;
    const image_width: u32 = 400;
    const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    // World
    let s1 = shapes::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = shapes::Sphere::new(vec3::Point3::new(0.0, -100.5, -1.0), 100.0);
    let mut world = shapes::HittableList::new();
    world.list_mut().push(Box::new(s1));
    world.list_mut().push(Box::new(s2));

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
    let mut pix_vec = Vec::<Pixel>::new();
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&r, &world);
            pix_vec.push(Pixel::new(i, j, color));
        }
    }

    let res = crate::image_writer::write_ppm(
        image_width,
        image_height,
        &pix_vec,
        std::path::Path::new("images/blue_sky.ppm"),
    );
    assert!(res.is_ok());

    let res = crate::image_writer::write_png(
        image_width,
        image_height,
        &pix_vec,
        std::path::Path::new("images/blue_sky.png"),
    );
    assert!(res.is_ok());

    println!("Done!")
}
