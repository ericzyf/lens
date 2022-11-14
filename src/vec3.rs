use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
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

    pub unsafe fn get_unchecked(&self, i: usize) -> &f64 {
        debug_assert!(i < 3);
        self.e.get_unchecked(i)
    }

    pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 3);
        self.e.get_unchecked_mut(i)
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
        for e in self.e.iter_mut() {
            *e *= t;
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        debug_assert!(t != 0.);
        for e in self.e.iter_mut() {
            *e /= t;
        }
    }
}
