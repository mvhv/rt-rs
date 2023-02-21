use nalgebra::{Vector3, Point3, Unit, vector};
use tracing::{span, Level};

use crate::Scalar;
use crate::colour;
use crate::material;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T>
where
    T: Scalar
{
    origin: Point3<T>,
    orientation: Unit<Vector3<T>>,
    attenuation: Vector3<T>,
    /// refractive index of the current medium
    /// maybe convert to a struct later for volumetric calculations
    medium: T
}

impl<T> Default for Ray<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self {
            origin: Point3::origin(),
            orientation: -Vector3::z_axis(),
            attenuation: colour::black(),
            medium: T::from_float(material::refractive_index::AIR)
        }
    }
}

impl<T> Ray<T>
where
    T: Scalar
{
    pub fn from_orientation_attenuated(origin: Point3<T>, orientation: Vector3<T>, attenuation: Vector3<T>) -> Self {
        Self {
            origin: origin,
            orientation: Unit::new_normalize(orientation),
            attenuation: attenuation,
            ..Default::default()
        }
    }

    pub fn from_orientation(origin: Point3<T>, orientation: Vector3<T>) -> Self {
        Self::from_orientation_attenuated(origin, orientation, Vector3::zeros())
    }

    pub fn from_focus_point_attenuated(origin: Point3<T>, focus: Point3<T>, attenuation: Vector3<T>) -> Self {
        Self::from_orientation_attenuated(origin, focus - origin, attenuation)
    }

    pub fn from_focus_point(origin: Point3<T>, focus: Point3<T>) -> Self {
        Self::from_focus_point_attenuated(origin, focus, Vector3::zeros())
    }

    pub fn project(&self, depth: T) -> Point3<T> {
        self.origin + (self.orientation.into_inner() * depth)
    }

    pub fn background_colour(&self) -> Vector3<T> {
        let y = self.orientation.y;
        let t = T::HALF * (y + T::one());
        // (colour::white() * (T::one() - t)) + (colour::light_blue() * t)
        // (colour::light_blue() * (T::one() - t)) + (colour::white() * t)
        (colour::red() * (T::one() - t)) + (colour::white() * t)
    }

    pub fn colour(&self) -> Vector3<T> {
        self.background_colour().component_mul(&self.gain())
    }

    pub fn origin(&self) -> Point3<T> {
        self.origin
    }

    pub fn orientation(&self) -> Unit<Vector3<T>> {
        self.orientation
    }

    pub fn attenuation(&self) -> Vector3<T> {
        self.attenuation
    }

    pub fn gain(&self) -> Vector3<T> {
        colour::white() - self.attenuation
    }

    pub fn medium(&self) -> T {
        self.medium
    }

    pub fn with_medium(self, medium: T) -> Self {
        Self {medium, ..self}
    }

    pub fn x(&self) -> T {
        self.orientation.x
    }
    
    pub fn y(&self) -> T {
        self.orientation.y
    }
    
    pub fn z(&self) -> T {
        self.orientation.z
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::{point, vector};

    #[test]
    pub fn test_ray_projection() {
        // orientation is the pythagorean quadruple 2^2 + 3^2 + 6^2 = 7^2
        let ray = Ray::from_orientation(
            point![1., 1., 1.],
            vector![2., 3., 6.],
        );
        let depth = 2.0;
        // cast_point = origin + depth * orientation
        let cast_point = point![
            1. + depth * (2. / 7.),
            1. + depth * (3. / 7.),
            1. + depth * (6. / 7.)
        ];
        assert_eq!(ray.project(depth), cast_point);
    }
}