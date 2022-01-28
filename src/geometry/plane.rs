use nalgebra::{Vector3, Point3, Unit};

use crate::{Scalar, Material};
use crate::geometry::{Ray, Intersectable, Intersection};

#[derive(Debug)]
pub struct Plane<T>
where
    T: Scalar
{
    origin: Point3<T>,
    normal: Unit<Vector3<T>>
}

impl<T> Intersectable<T> for Plane<T>
where
    T: Scalar
{
    fn intersect(&self, ray: Ray<T>) -> Option<Intersection<T>> {
        todo!()
    }
    // fn material(&self) -> Material<T> {
    //     todo!()
    // }
    // fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>> {
    //     todo!()
    // }
}