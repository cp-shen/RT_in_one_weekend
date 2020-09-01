use crate::vec3;

struct ray {
    orig: vec3::Point3,
    dir: vec3::Vec3,
}

impl ray {
    fn at(&self, t: f32) -> vec3::Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
    }
}
