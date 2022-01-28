use nalgebra::{RealField, ClosedAdd, ClosedSub, ClosedMul, ClosedDiv};
use num_traits::Num;

pub trait Scalar: RealField + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + Num + Copy {
    const HALF: Self;

    fn from_float(val: f64) -> Self;
    fn scale_to_u8(self) -> u8;
}

impl Scalar for f64 {
    const HALF: Self = 0.5;

    fn from_float(val: f64) -> Self {
        val
    }

    fn scale_to_u8(self) -> u8 {
        (self.clamp(0.0, 1.0) * 255.0) as u8
    }
}

impl Scalar for f32 {
    const HALF: Self = 0.5;

    fn from_float(val: f64) -> Self {
        val as f32
    }

    fn scale_to_u8(self) -> u8 {
        (self.clamp(0.0, 1.0) * 255.0) as u8
    }
}