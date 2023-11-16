// use nalgebra::{Point3, Vector3, point, vector};

use crate::{geometry::{Aabb, Geometry}, Scalar};

struct BvhBox<T>(Aabb<T>, Geometry<T>) where T: Scalar;

struct Bvh<T>
where
    T: Scalar
{
    boxes: Vec<(Aabb<T>, Geometry<T>)>,
    tree: Vec<usize>,
    // infinite sized objects have to be tracked outside the bb
    unboxed: Vec<usize>,
}

