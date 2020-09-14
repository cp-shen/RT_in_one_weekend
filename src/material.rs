use crate::ray::Ray;
use crate::shapes::*;
use crate::vec3::*;

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
        todo!();
        //None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
