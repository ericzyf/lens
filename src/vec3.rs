use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        unsafe { *self.e.get_unchecked(0) }
    }

    pub fn y(&self) -> f64 {
        unsafe { *self.e.get_unchecked(1) }
    }

    pub fn z(&self) -> f64 {
        unsafe { *self.e.get_unchecked(2) }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let [x, y, z] = self.e;
        x * x + y * y + z * z
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub unsafe fn get_unchecked(&self, i: usize) -> &f64 {
        debug_assert!(i < 3);
        self.e.get_unchecked(i)
    }

    pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 3);
        self.e.get_unchecked_mut(i)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let [x, y, z] = self.e;
        Vec3::new(-x, -y, -z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        for i in 0..3 {
            unsafe {
                *self.get_unchecked_mut(i) += other.get_unchecked(i);
            }
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        for e in &mut self.e {
            *e *= t;
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        debug_assert!(t != 0.);
        for e in &mut self.e {
            *e /= t;
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self += -other;
        self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut other: Vec3) -> Self::Output {
        other *= self;
        other
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        t * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, t: f64) -> Self {
        self /= t;
        self
    }
}
