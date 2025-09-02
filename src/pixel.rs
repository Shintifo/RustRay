use crate::vector::Vec3;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Index, Mul, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) struct PixelRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelRGB {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
        PixelRGB { r, g, b }
    }
    pub(crate) fn new_vec(vec: Vec3) -> Self {
        PixelRGB::new(
            vec.x.clamp(0.0, 255.0) as u8,
            vec.y.clamp(0.0, 255.0) as u8,
            vec.z.clamp(0.0, 255.0) as u8,
        )
    }
    pub(crate) fn is_white(&self) -> bool {
        self.r == 255 && self.g == 255 && self.b == 255
    }

    pub(crate) fn white() -> Self {
        PixelRGB::new(255, 255, 255)
    }

    pub(crate) fn r(&self) -> u8 {
        self.r
    }

    pub(crate) fn g(&self) -> u8 {
        self.g
    }

    pub(crate) fn b(&self) -> u8 {
        self.b
    }
}

impl Display for PixelRGB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Add for PixelRGB {
    type Output = PixelRGB;

    fn add(self, other: PixelRGB) -> Self::Output {
        let r = (self.r as u16 + other.r as u16).clamp(0, 255) as u8;
        let g = (self.g as u16 + other.g as u16).clamp(0, 255) as u8;
        let b = (self.b as u16 + other.b as u16).clamp(0, 255) as u8;
        PixelRGB::new(r, g, b)
    }
}

impl Sub for PixelRGB {
    type Output = PixelRGB;
    fn sub(self, other: PixelRGB) -> Self::Output {
        let r = (self.r as i16 - other.r as i16).clamp(0, 255) as u8;
        let g = (self.g as i16 - other.g as i16).clamp(0, 255) as u8;
        let b = (self.b as i16 - other.b as i16).clamp(0, 255) as u8;
        PixelRGB::new(r, g, b)
    }
}

impl AddAssign<&PixelRGB> for PixelRGB {
    fn add_assign(&mut self, other: &Self) {
        self.r = (self.r as u16 + other.r as u16).clamp(0, 255) as u8;
        self.g = (self.g as u16 + other.g as u16).clamp(0, 255) as u8;
        self.b = (self.b as u16 + other.b as u16).clamp(0, 255) as u8;
    }
}

impl SubAssign<&PixelRGB> for PixelRGB {
    fn sub_assign(&mut self, other: &Self) {
        self.r = (self.r as i16 - other.r as i16).clamp(0, 255) as u8;
        self.g = (self.g as i16 - other.g as i16).clamp(0, 255) as u8;
        self.b = (self.b as i16 - other.b as i16).clamp(0, 255) as u8;
    }
}

impl Mul<f32> for PixelRGB {
    type Output = PixelRGB;
    fn mul(self, scalar: f32) -> Self::Output {
        let r = (self.r as f32 * scalar).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * scalar).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * scalar).clamp(0.0, 255.0) as u8;
        PixelRGB::new(r, g, b)
    }
}

impl Mul<PixelRGB> for f32 {
    type Output = PixelRGB;
    fn mul(self, pixel: PixelRGB) -> Self::Output {
        pixel * self
    }
}

impl Mul<f32> for &PixelRGB {
    type Output = PixelRGB;
    fn mul(self, scalar: f32) -> Self::Output {
        *self * scalar
    }
}

impl Mul<&PixelRGB> for f32 {
    type Output = PixelRGB;
    fn mul(self, pixel: &PixelRGB) -> Self::Output {
        *pixel * self
    }
}
