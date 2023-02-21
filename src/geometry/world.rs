use nalgebra::point;
use tracing::trace;

use crate::{Scalar, Material, colour, material};
use crate::geometry::{Ray, Intersectable, Intersection, Geometry};

use super::{Sphere, Plane};

// #[derive(Debug, Clone)]
// pub struct DynWorld<T>
// where
//     T: Scalar
// {
//     objects: Vec<Box<dyn Intersectable<T>>>,
// }

// impl<T> Intersectable<T> for DynWorld<T>
// where
//     T: Scalar
// {
//     fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
//         self.objects
//             .iter()
//             .filter_map(|s| s.intersect(ray, min_depth, max_depth))
//             .reduce(|acc, next| {
//                 let prev_length = (acc.point() - ray.origin()).norm();
//                 let next_length = (next.point() - ray.origin()).norm();
//                 if prev_length > next_length { next } else { acc }
//             })
//     }
// }

#[derive(Debug, Clone)]
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
        let res = self.objects
            .iter()
            .filter_map(|geom| {
                trace!("Searching: {ray:?} with {geom:?}");
                match geom {
                    Geometry::Sphere(s) => s.intersect(ray, min_depth, max_depth),
                    Geometry::Plane(p) => p.intersect(ray, min_depth, max_depth),
                }
            }).reduce(|acc, next| {
                let prev_length = (acc.point() - ray.origin()).norm();
                let next_length = (next.point() - ray.origin()).norm();
                if prev_length > next_length { next } else { acc }
            });
        trace!("Found: {res:?}");
        res
    }
}

pub fn three_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = StaticWorld::default();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.2), T::from_float(0.5), -T::TWO],
            T::from_float(0.6),
            Material::mirror(),
            // Material::simple_diffuse_colour(colour::white()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(-1.25), T::from_float(0.75-0.5), -T::one()],
            T::from_float(0.75),
            Material::simple_diffuse_colour(colour::light_pink()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.6), T::from_float(0.25-0.5), -T::from_float(0.7)],
            T::from_float(0.25),
            Material::simple_diffuse_colour(colour::light_green()),
        )
    );
    world.push_plane(Plane::default());
    world
}

pub fn five_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = StaticWorld::default();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.2), T::from_float(0.5), -T::TWO],
            T::from_float(0.6),
            Material::mirror(),
            // Material::simple_diffuse_colour(colour::white()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![-T::HALF, T::HALF, T::one()],
            T::from_float(0.5),
            Material::simple_diffuse_colour(colour::bright_blue()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(-1.25), T::from_float(0.75-0.5), -T::one()],
            T::from_float(0.75),
            Material::mirror(),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.6), T::from_float(0.25-0.5), -T::from_float(0.7)],
            T::from_float(0.25),
            Material::glass(),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![-T::from_float(0.3), -T::from_float(0.4), -T::from_float(0.6)],
            T::from_float(0.1),
            Material::simple_diffuse_colour(colour::yellow()),
        )
    );
    world.push_plane(Plane::default());
    world
}

pub fn six_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = five_sphere_scene();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(-0.1), T::from_float(-0.1), -T::from_float(0.9)],
            T::from_float(0.25),
            Material::glass(),
        )
    );
    world
}

pub fn seven_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = six_sphere_scene();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(4.0), T::from_float(4.5), -T::from_float(10.0)],
            T::from_float(5.0),
            Material::simple_diffuse_colour(colour::bright_purple()),
        )
    );
    world
}

pub fn eight_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = seven_sphere_scene();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.2), T::from_float(0.2-0.5), -T::from_float(1.5)],
            T::from_float(0.2),
            Material::simple_diffuse_colour(colour::bright_red()),
        )
    );
    world
}

pub fn ten_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = eight_sphere_scene();
    world.push_sphere(
        Sphere::new(
            point![T::from_float(0.21), T::from_float(0.08-0.5), -T::from_float(0.76)],
            T::from_float(0.08),
            Material::simple_diffuse_colour(colour::bright_orange()),
        )
    );
    world.push_sphere(
        Sphere::new(
            point![T::from_float(1.7), T::from_float(0.6-0.5), -T::from_float(1.8)],
            T::from_float(0.6),
            Material::simple_diffuse_colour(colour::bright_green()),
        )
    );
    world
}

pub fn one_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = StaticWorld::default();
    world.push_sphere(Sphere::default());
    world.push_plane(Plane::default());
    world
}

pub fn glass_sphere_scene<T: Scalar>() -> StaticWorld<T> {
    let mut world = StaticWorld::default();
    world.push_sphere(
        Sphere::new(
            point![T::zero(), T::from_float(0.1), -T::one()],
            T::from_float(0.5),
            Material::glass(),
            )
        );
    world.push_plane(Plane::default());
    world
}