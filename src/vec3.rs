use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f32, f32, f32);

pub type Color = Vec3;
pub type Point3 = Vec3;

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self + -rhs
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Vec3 {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }

    fn z(&self) -> f32 {
        self.2
    }

    fn length_sqared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn length(&self) -> f32 {
        self.length_sqared().sqrt()
    }

    fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn cross(&self, other: &Self) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    fn unit_vector(&self) -> Vec3 {
        assert!(self.length() > 0_f32, "Vector length is zero!");

        *self * (1_f32 / self.length())
    }

    fn float_color_to_8bit(&self) -> Vec<u8> {
        let r = self.0.min(1_f32).max(0_f32) * 255_f32;
        let g = self.0.min(1_f32).max(0_f32) * 255_f32;
        let b = self.0.min(1_f32).max(0_f32) * 255_f32;
        vec![r.round() as u8, g.round() as u8, b.round() as u8]
    }
}
