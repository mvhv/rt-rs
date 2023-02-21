use nalgebra::{Point3, Vector3, point, vector};

use crate::geometry::Aabb;

struct Bvh {
    boxes: Vec<Aabb>
}

