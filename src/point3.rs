use super::Vec3;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

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

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, Self(v): Self) -> Self::Output {
        let Self(u) = self;
        u - v
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        self + (-rhs)
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = &self.0;
        write!(f, "({}, {}, {})", v.x(), v.y(), v.z())
    }
}
