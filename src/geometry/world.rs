use crate::Scalar;
use crate::geometry::{Ray, Intersectable, Intersection, Geometry};

pub struct DynWorld<T>
where
    T: Scalar
{
    objects: Vec<Box<dyn Intersectable<T>>>,
}

impl<T> Intersectable<T> for DynWorld<T>
where
    T: Scalar
{
    fn intersect(&self, ray: Ray<T>) -> Option<Intersection<T>> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray))
            .reduce(|acc, next| {
                let prev_length = (acc.point() - ray.origin).norm();
                let next_length = (next.point() - ray.origin).norm();
                if prev_length > next_length { next } else { acc }
            })
    }
}

#[derive(Debug)]
pub struct StaticWorld<T>
where
    T: Scalar
{
    objects: Vec<Geometry<T>>
}

impl<T> Default for StaticWorld<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self { objects: Vec::new() }
    }
}

impl<T> StaticWorld<T>
where
    T: Scalar
{
    pub fn push(&mut self, new_object: Geometry<T>) {
        self.objects.push(new_object);
    }

    pub fn push_sphere(&mut self, new_sphere: super::Sphere<T>) {
        self.objects.push(Geometry::Sphere(new_sphere))
    }
}

impl<T> Intersectable<T> for StaticWorld<T>
where
    T: Scalar
{
    fn intersect(&self, ray: Ray<T>) -> Option<Intersection<T>> {
        self.objects
            .iter()
            .filter_map(|geom| match geom {
                Geometry::Sphere(s) => s.intersect(ray),
                Geometry::Plane(p) => p.intersect(ray),
            })
            .reduce(|acc, next| {
                let prev_length = (acc.point() - ray.origin).norm();
                let next_length = (next.point() - ray.origin).norm();
                if prev_length > next_length { next } else { acc }
            })
    }
}