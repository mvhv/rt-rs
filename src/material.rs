use nalgebra::{Vector3};

use crate::{colour, Scalar};

#[derive(Debug, Clone, Copy)]
pub struct Material<T>
where
    T: Scalar
{
    colour: Vector3<T>,
    specularity: T,
    diffusivity: T,
    transmissibility: T,
    refractive_index: T,
}

impl<T> Default for Material<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self {
            colour: colour::red(),
            specularity: T::zero(),
            diffusivity: T::zero(),
            transmissibility: T::zero(),
            refractive_index: T::zero(),
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
        Self { colour, diffusivity: T::from_float(0.5), ..Default::default() }
    }
}