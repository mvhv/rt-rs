use nalgebra::{Vector3, vector};

use crate::Scalar;

pub fn rgb<T: Scalar>(r: T, g: T, b: T) -> Vector3<T> {
    vector![r, g, b]
}

pub fn red<T: Scalar>() -> Vector3<T> {
    vector![T::one(), T::zero(), T::zero()]
}

pub fn green<T: Scalar>() -> Vector3<T> {
    vector![T::zero(), T::one(), T::zero()]
}
pub fn blue<T: Scalar>() -> Vector3<T> {
    vector![T::zero(), T::zero(), T::one()]
}

pub fn white<T: Scalar>() -> Vector3<T> {
    vector![T::one(), T::one(), T::one()]
}

pub fn black<T: Scalar>() -> Vector3<T> {
    vector![T::zero(), T::zero(), T::zero()]
}

pub fn grey<T: Scalar>() -> Vector3<T> {
    vector![T::from_float(0.5), T::from_float(0.5), T::from_float(0.5)]
}

pub fn light_blue<T: Scalar>() -> Vector3<T> {
    vector![T::from_float(0.5), T::from_float(0.7), T::from_float(1.0)]
}
