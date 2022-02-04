use nalgebra::point;

use crate::{Scalar, Material, colour};
use crate::geometry::{Ray, Intersectable, Intersection, Geometry};

use super::{Sphere, Plane};

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
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray, min_depth, max_depth))
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

    pub fn push_plane(&mut self, new_plane: super::Plane<T>) {
        self.objects.push(Geometry::Plane(new_plane))
    }
}

impl<T> Intersectable<T> for StaticWorld<T>
where
    T: Scalar
{
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
        self.objects
            .iter()
            .filter_map(|geom| match geom {
                Geometry::Sphere(s) => s.intersect(ray, min_depth, max_depth),
                Geometry::Plane(p) => p.intersect(ray, min_depth, max_depth),
            })
            .reduce(|acc, next| {
                let prev_length = (acc.point() - ray.origin).norm();
                let next_length = (next.point() - ray.origin).norm();
                if prev_length > next_length { next } else { acc }
            })
    }
}

pub fn three_sphere_world<T: Scalar>() -> StaticWorld<T> {
    let mut world = StaticWorld::default();
    world.push_sphere(Sphere::default());
    world.push_sphere(
        Sphere::new(
            point![T::from_float(-1.5), T::zero(), -T::one()],
            T::from_float(0.5),
            Material::simple_diffuse_colour(colour::green()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(1.5), T::zero(), -T::one()],
            T::from_float(0.5),
            Material::simple_diffuse_colour(colour::blue()),
        )
    );
    world.push_plane(Plane::default());
    world
}