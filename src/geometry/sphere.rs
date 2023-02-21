use std::option::Option;

use nalgebra::{Vector3, Point3, Unit, point};

use crate::{
    colour,
    geometry::{
        AABB,
        Intersectable,
        Intersection,
        Ray
    },
    Material,
    Scalar,
};

#[derive(Debug, Clone)]
pub struct Sphere<T>
where
    T: Scalar
{
    center: Point3<T>,
    radius: T,
    material: Material<T>,
}

impl<T> Default for Sphere<T>
where
    T: Scalar
{
    fn default() -> Self {
        Self {
            center: point![T::zero(), T::zero(), -T::one()],
            radius: T::from_float(0.5),
            material: Material::simple_diffuse_colour(colour::red()),
        }
    }
}

impl<T> Sphere<T>
where
    T: Scalar
{
    pub fn new(center: Point3<T>, radius: T, material: Material<T>) -> Self {
        Self { center, radius, material }
    }

    pub fn material(&self) -> Material<T> {
        self.material
    }

    pub fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>> {
        Unit::new_normalize(point - self.center)
    }

    pub fn with_material(self, material: Material<T>) -> Self {
        Self { material, ..Default::default() }
    }
}

impl<T> Intersectable<T> for Sphere<T>
where
    T: Scalar
{
    #[cfg(not(feature = "optimised_intersection"))]
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
        // trace!("interesecting ray {ray:?} with sphere {self:?}");
        let origin_to_center = ray.origin() - self.center;
        // quadratic method
        let a = ray.orientation().dot(&ray.orientation());
        let b = T::from_float(2.0) * origin_to_center.dot(&ray.orientation());
        let c = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;
        let descriminant = b*b - T::from_float(4.0)*a*c;
        // if descrim is below zero then no real valued solution (i.e. no intersect)
        if descriminant >= T::zero() {
            // why am I only checking the positive value here?
            // does this disallow intersections behind the camera?
            let divisor = T::from_float(2.0) * a;
            let descriminant_root = descriminant.sqrt();
            let root_low = (-b - descriminant_root) / divisor;
            let root_high = (-b + descriminant_root) / divisor;
            let root = if root_low > T::zero() {root_low} else {root_high};
            if root >= min_depth && root <= max_depth {   
                let point = ray.project(root);
                // let incident = ray.orientation();
                let normal = self.normal(point);
                let material = self.material();
                Some(Intersection::new(point, ray, normal, material))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[cfg(feature = "optimised_intersection")]
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>> {
        // TODO: Fix this stupid shit
        // debug!("Optimised Intersection");
        // simplified using the known constant factor in b and that the ray is unit length
        let origin_to_center = ray.origin - self.center;
        let half_b = origin_to_center.dot(&ray.orientation); //common factors cancel out
        let c = origin_to_center.norm() - self.radius * self.radius; //self dot is equivalent to norm/length
        let descriminant = half_b*half_b - c;
        if descriminant < T::zero() {
            None
        } else {
            let root = -(half_b + descriminant.sqrt());
            let point = ray.project(root);
            let incident = ray.orientation;
            let normal = self.normal(point);
            let material = self.material();
            Some(Intersection::new(point, incident, normal, material))
        }
    }

    fn bounding_box(&self) -> Option<AABB<T>> {
        None
    }
}

// pub enum Root<T>
// where
//     T: Scalar
// {
//     Zero,
//     One(T),
//     Two(T),
// }

// pub struct SphereVec<T: Scalar>(Vec<Sphere<T>>);

// impl<T> Intersectable<T> for SphereVec<T>
// where
//     T: Scalar
// {
//     fn intersect(&self, ray: Ray<T>) -> Option<Intersection<T>> {
//         self.0
//             .iter()
//             .filter_map(|s| s.intersect(ray))
//             .reduce(|acc, next| {
//                 let prev_length = (acc.point() - ray.origin).norm();
//                 let next_length = (next.point() - ray.origin).norm();
//                 if prev_length > next_length { next } else { acc }
//             })
//     }

//     fn material(&self) -> Material<T> {
//         todo!()
//     }

//     fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>> {
//         todo!()
//     } 
// }