use nalgebra::{point, vector, Point3, Vector3, Matrix};

use crate::{
    geometry::{Intersectable, Intersection, Ray},
    Scalar,
};

// not sure what works best yet
// three min-max tuples (one for each asis) 
// struct AABB<T> {
//     x: (T, T),
//     y: (T, T),
//     z: (T, T),
// }
// struct Interval<T>(T, T);

// fn interval_overlap<T: Scalar>(x: Interval<T>, y: Interval<T>, z: Interval<T>) -> bool {
//     true
// }

// don't think I actually want the AABB to be "Intersectable"
// the BVH should own the entire graph and these structs should
// only be metadata
// impl<T> Intersectable<T> for AABB<T>
// where
//     T: Scalar
// {

// }

/// this two vector approach works well with nalgebra
pub struct Aabb<T>
where
    T: Scalar
{
    min: Point3<T>,
    max: Point3<T>,
}

impl<T> Aabb<T>
where
    T: Scalar
{
    pub fn intersect(&self, ray: Ray<T>, min_depth: T) -> bool {
        let t0 = (self.min - ray.origin()).component_div(&ray.orientation());
        let t1 = (self.max - ray.origin()).component_div(&ray.orientation());
        let (t_min, t_max) = t0.inf_sup(&t1);
        // maybe need to check for nans, not sure how inf_sup handles it.
        let (t_low, t_hi) = (Matrix::max(&t_min), Matrix::min(&t_max));
        // NaN indicates the grazing case, as long as we slightly inflate the bounding box then false here is correct
        t_hi >= t_low && t_hi >= min_depth
    }
}