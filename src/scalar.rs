use std::ops::Mul;

use nalgebra::{RealField, ClosedAdd, ClosedSub, ClosedMul, ClosedDiv};
use num_traits::Num;

pub trait Scalar: RealField + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + Num + From<f32> + Copy + Send + Sync + 'static {
    const HALF: Self;
    const INF: Self;
    const TWO: Self;

    fn from_float(val: f64) -> Self;
    fn scale_to_u8(self) -> u8;
}

impl Scalar for f64 {
    const HALF: Self = 0.5;
    const TWO: Self = 2.0;
    const INF: Self = Self::INFINITY;

    fn from_float(val: f64) -> Self {
        val
    }

    fn scale_to_u8(self) -> u8 {
        self.clamp(0.0, 1.0)
            .mul(255.0) as u8
    }
}

impl Scalar for f32 {
    const HALF: Self = 0.5;
    const TWO: Self = 2.0;
    const INF: Self = Self::INFINITY;

    fn from_float(val: f64) -> Self {
        val as f32
    }

    fn scale_to_u8(self) -> u8 {
        self.clamp(0.0, 1.0)
            .mul(255.0) as u8
    }
}