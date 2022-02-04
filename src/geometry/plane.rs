use nalgebra::{Vector3, Point3, Unit, vector, point};

use crate::{Scalar, Material, colour};
use crate::geometry::{Ray, Intersectable, Intersection};

#[derive(Debug)]
pub struct Plane<T>
where
    T: Scalar
{
    origin: Point3<T>,
    normal: Unit<Vector3<T>>,
    material: Material<T>,
}

impl<T> Plane<T>
where
    T: Scalar
{
    pub fn new(origin: Point3<T>, normal: Unit<Vector3<T>>, material: Material<T>) -> Self {
        Self { origin, normal, material }
    }
}

impl<T> Default for Plane<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self {
            origin: point![T::zero(), T::from_float(-5.0), T::zero()],
            normal: Unit::new_normalize(vector![T::zero(), T::one(), T::zero()]),
            material: Material::simple_diffuse_colour(colour::black()),
        }
    }
}

impl<T> Intersectable<T> for Plane<T>
where
    T: Scalar
{
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
        // plane equation (p - p0).n = 0
        // line equation p = l0 + l*d
        // l.n != 0 for an intersection
        let ray_dot_normal = ray.orientation.dot(&self.normal);
        if ray_dot_normal == T::zero() {
            None
        } else {
            // d = ((p0 - l0).n) / l.n
            let depth = (self.origin - ray.origin).dot(&self.normal) / ray_dot_normal;
            if depth >= min_depth && depth <= max_depth {
                let point = ray.project(depth);
                Some(Intersection::new(point, ray.orientation, self.normal, self.material))
            } else {
                None
            }
            
        }
    }
    // fn material(&self) -> Material<T> {
    //     todo!()
    // }
    // fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>> {
    //     todo!()
    // }
}