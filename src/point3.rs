use super::Vec3;
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct Point3(pub Vec3);

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(mut self, rhs: Vec3) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs;
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = &self.0;
        write!(f, "({}, {}, {})", v.x(), v.y(), v.z())
    }
}
