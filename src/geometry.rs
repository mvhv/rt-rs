pub mod ray;
pub mod intersect;
pub mod sphere;
pub mod plane;
pub mod world;
pub mod aabb;
pub mod bvh;

pub use ray::Ray;
pub use intersect::{Intersectable, Intersection};
pub use sphere::Sphere;
pub use plane::Plane;
pub use aabb::Aabb;

use crate::Scalar;

// use core::fmt::Debug;

// #[derive(Clone, Debug)]
pub enum Geometry<T>
where
    T: Scalar
{
    Sphere(Sphere<T>),
    Plane(Plane<T>),
}