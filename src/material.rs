use nalgebra::Vector3;

use crate::{colour, Scalar};

pub mod refractive_index {
    pub const AIR: f64 = 1.0;
    pub const WATER: f64 = 4.0/3.0;
    pub const GLASS: f64 = 1.5;
    pub const DIAMOND: f64 = 2.4;
    pub const MAGIC: f64 = 100.0;
}

#[derive(Debug, Clone, Copy)]
pub struct Material<T>
where
    T: Scalar
{
    colour: Vector3<T>,
    absorptivity: T,
    specularity: T,
    diffusivity: T,
    transmissibility: T,
    refractive_index: T,
    // temporary value for colouring objects
    pub checkerboard: bool,
}

impl<T> Default for Material<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self {
            colour: colour::red(),
            absorptivity: T::from_float(0.2),
            specularity: T::zero(),
            diffusivity: T::one(),
            transmissibility: T::zero(),
            refractive_index: T::one(),
            checkerboard: false,
        }
    }
}

impl<T> Material<T>
where
    T: Scalar
{
    pub fn colour(&self) -> Vector3<T> {
        self.colour
    }

    pub fn simple_diffuse_colour(colour: Vector3<T>) -> Self {
        Self { colour, diffusivity: T::one(), ..Default::default() }
    }

    pub fn mirror() -> Self {
        Self {
            colour: colour::white(),
            absorptivity: T::from_float(0.05),
            specularity: T::from_float(0.95),
            diffusivity: T::from_float(0.05),
            ..Default::default()
        }
    }

    pub fn checkerboard() -> Self {
        Self {
            colour: colour::white(),
            checkerboard: true,
            specularity: T::from_float(0.3),
            diffusivity: T::from_float(0.7),
            ..Default::default()
        }
    }

    pub fn glass() -> Self {
        Self {
            colour: colour::white(),
            absorptivity: T::from_float(0.0),
            specularity: T::from_float(0.8),
            diffusivity: T::from_float(0.2),
            transmissibility: T::from_float(1.0),
            refractive_index: T::from_float(refractive_index::GLASS),
            ..Default::default()
        }
    }

    /// a vector representing the fraction of absorbed light for each colour channel
    pub fn attenuation(&self) -> Vector3<T> {
        colour::white() - self.albedo()
    }

    /// a vector representing the fraction of reflected light for each colour channel
    pub fn albedo(&self) -> Vector3<T> {
        self.colour * (T::one() - self.absorptivity)
    }

    /// coherency is the probability of a specular reflection
    pub fn coherency(&self) -> T {
        if self.specularity <= T::zero() {
            T::zero()
        } else {
            self.specularity.div(self.specularity + self.diffusivity)
        }
    }

    pub fn transmissibility(&self) -> T {
        self.transmissibility
    }

    pub fn refractive_index(&self) -> T {
        self.refractive_index
    }
}