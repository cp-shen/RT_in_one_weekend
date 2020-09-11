#[macro_use]
extern crate derive_new;

mod camera;
mod image_writer;
mod random;
mod ray;
mod shapes;
mod vec3;

use crate::camera::*;
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
    const sample_per_pixel: u32 = 100;

    // World
    let s1 = shapes::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = shapes::Sphere::new(vec3::Point3::new(0.0, -100.5, -1.0), 100.0);
    let mut world = shapes::HittableList::new();
    world.list_mut().push(Box::new(s1));
    world.list_mut().push(Box::new(s2));

    // Camera
    let cam = Camera::default();

    // Render
    let mut pix_vec = Vec::<Pixel>::new();
    let mut pix_vec_multisample = Vec::<Pixel>::new();
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = cam.get_ray(u, v);
            let color = ray_color(&r, &world);
            pix_vec.push(Pixel::new(i, j, color));

            let color_multisample = (0..sample_per_pixel)
                .map(|_| {
                    let u = (i as f32 + random::random_float())
                        / (image_width - 1) as f32;
                    let v = (j as f32 + random::random_float())
                        / (image_height - 1) as f32;
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world) * (1.0 / sample_per_pixel as f32)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |c1, c2| c1 + c2);
            pix_vec_multisample.push(Pixel::new(i, j, color_multisample));
        }
    }

    let res = crate::image_writer::write_png(
        image_width,
        image_height,
        &pix_vec,
        std::path::Path::new("images/blue_sky.png"),
    );
    assert!(res.is_ok());

    let res = crate::image_writer::write_png(
        image_width,
        image_height,
        &pix_vec_multisample,
        std::path::Path::new("images/blue_sky_multisample.png"),
    );
    assert!(res.is_ok());

    println!("Done!")
}
