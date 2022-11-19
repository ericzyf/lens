use super::Vec3;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        debug_assert!(r >= 0. && r <= 1.);
        debug_assert!(g >= 0. && g <= 1.);
        debug_assert!(b >= 0. && b <= 1.);

        Color(Vec3::new(r, g, b))
    }
}

impl Color {
    pub fn gamma_correction(self) -> Self {
        const INV_GAMMA: f64 = 1. / 2.2;
        let Color(v) = self;
        Color::new(
            v.x().powf(INV_GAMMA),
            v.y().powf(INV_GAMMA),
            v.z().powf(INV_GAMMA),
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color(self.0 + other.0)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color(self.0 * other.0)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, Color(v): Color) -> Self::Output {
        Color(self * v)
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(Color(v): Color) -> Self {
        let r = (v.x() * 255.999) as u8;
        let g = (v.y() * 255.999) as u8;
        let b = (v.z() * 255.999) as u8;

        image::Rgb([r, g, b])
    }
}
