use crate::ray::Ray;
use crate::shapes::*;
use crate::vec3::*;
use crate::random;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(new, getset::CopyGetters)]
pub struct Lambertian {
    #[getset(get_copy = "pub")]
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal() + random::random_unit_vector();
        let scatted = Ray::new(rec.point(), scatter_direction);
        let attenuation = self.albedo;
        Some((scatted, attenuation))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

pub struct Metal {}
