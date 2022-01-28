use nalgebra::{Vector3, Point3, Unit};
use tracing::{span, Level};

use crate::Scalar;
use crate::colour;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T>
where
    T: Scalar
{
    pub origin: Point3<T>,
    pub orientation: Unit<Vector3<T>>,
    //reference frame
}

impl<T> Ray<T>
where
    T: Scalar
{
    pub fn from_orientation(origin: Point3<T>, orientation: Vector3<T>) -> Self {
        Self {
            origin: origin,
            orientation: Unit::new_normalize(orientation),
        }
    }

    pub fn from_focus_point(origin: Point3<T>, focus: Point3<T>) -> Self {
        Self {
            origin: origin,
            orientation: Unit::new_normalize(focus - origin),
        }
    }

    pub fn project(&self, depth: T) -> Point3<T> {
        self.origin + (self.orientation.into_inner() * depth)
    }

    pub fn background_colour(&self) -> Vector3<T> {
        let y = self.orientation.y;
        let t = T::HALF * (y + T::one());
        (colour::white() * (T::one() - t)) + (colour::light_blue() * t)
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