use core::fmt::Debug;

use nalgebra::{Vector3, Point3, Unit};

use crate::{Scalar, Material};
use crate::geometry::Ray;

#[derive(PartialEq, Clone, Copy)]
pub enum Face {
    Front,
    Back,
}

pub struct Intersection<T>
where
    T: Scalar
{
    point: Point3<T>,
    normal: Unit<Vector3<T>>,
    material: Material<T>,
    face: Face,
    // incident: Unit<Vector3<T>>,
}

impl<T> Intersection<T>
where
    T: Scalar
{
    pub fn new(point: Point3<T>, incident: Unit<Vector3<T>>, normal: Unit<Vector3<T>>, material: Material<T>) -> Self {
        let face = if normal.dot(&incident) >= T::zero() { Face::Front } else { Face::Back };
        Intersection { point, normal, material, face }
    }

    pub fn point(&self) -> Point3<T> {
        self.point
    }

    pub fn normal(&self) -> Unit<Vector3<T>> {
        self.normal
    }

    pub fn material(&self) -> Material<T> {
        self.material
    }

    pub fn face(&self) -> Face {
        self.face
    }

    pub fn front_face(&self) -> bool {
        self.face == Face::Front
    }

    pub fn back_face(&self) -> bool {
        self.face == Face::Back
    }
}

pub trait Intersectable<T>
where
    T: Scalar + Debug
{
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>>;
    // fn material(&self) -> Material<T>;
    // fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>>;
}