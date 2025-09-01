use crate::vector::Vec3;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Deref, DerefMut, Index, Mul, Sub, SubAssign};

//TODO: clamp values at operands
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) struct PixelRGB(Vec3);

impl PixelRGB {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        PixelRGB(Vec3::new(
            x.clamp(0.0, 255.99),
            y.clamp(0.0, 255.99),
            z.clamp(0.0, 255.99),
        ))
    }
    pub(crate) fn is_white(&self) -> bool {
        self.0 == Vec3::new(1.0, 1.0, 1.0)
    }

    pub(crate) fn white() -> Self {
        PixelRGB::new(1.0, 1.0, 1.0)
    }

    fn new_vec(vec: Vec3) -> Self {
        PixelRGB::new(vec.x, vec.y, vec.z)
    }
}

impl Display for PixelRGB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.x as u8),
            (self.y as u8),
            (self.z as u8)
        )
    }
}

impl Deref for PixelRGB {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PixelRGB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for PixelRGB {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for PixelRGB {
    type Output = PixelRGB;

    fn add(self, other: PixelRGB) -> Self::Output {
        PixelRGB::new_vec(self.0 + other.0)
    }
}

impl Sub for PixelRGB {
    type Output = PixelRGB;
    fn sub(self, other: PixelRGB) -> Self::Output {
        PixelRGB::new_vec(self.0 - other.0)
    }
}

impl AddAssign<&PixelRGB> for PixelRGB {
    fn add_assign(&mut self, other: &Self) {
        self.0 += other.0;
    }
}

impl SubAssign<&PixelRGB> for PixelRGB {
    fn sub_assign(&mut self, other: &Self) {
        self.0 -= other.0;
    }
}

impl Mul<f32> for PixelRGB {
    type Output = PixelRGB;
    fn mul(self, scalar: f32) -> Self::Output {
        PixelRGB(self.0 * scalar)
    }
}

impl Mul<PixelRGB> for f32 {
    type Output = PixelRGB;
    fn mul(self, pixel: PixelRGB) -> Self::Output {
        PixelRGB(self * pixel.0)
    }
}

impl Mul<&PixelRGB> for f32 {
    type Output = PixelRGB;
    fn mul(self, pixel: &PixelRGB) -> Self::Output {
        PixelRGB(self * pixel.0)
    }
}
