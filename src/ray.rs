use crate::vec3::*;

#[derive(getset::CopyGetters)]
pub struct Ray {
    #[getset(get_copy = "pub")]
    orig: Point3,
    #[getset(get_copy = "pub")]
    dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self {
            orig,
            dir: dir.unit_vector(),
        }
    }
}

#[cfg(test)]
mod tests {}
