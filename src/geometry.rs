pub mod ray;
pub mod intersect;
pub mod sphere;
pub mod plane;
pub mod world;

pub use ray::Ray;
pub use intersect::{Intersectable, Intersection};
pub use sphere::Sphere;
pub use plane::Plane;

use crate::Scalar;

#[derive(Debug)]
pub enum Geometry<T>
where
    T: Scalar
{
    Sphere(Sphere<T>),
    Plane(Plane<T>),
}