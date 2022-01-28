use nalgebra::{Point3, Vector3, Unit, vector};
use tracing::debug;

use crate::{Scalar, image::AspectRatio, geometry::Ray};

#[derive(Debug)]
struct Viewport<T>
where
    T: Scalar
{
    /// Lower left corner of the viewport
    origin: Point3<T>,
    /// Horizontal basis vector of the viewport
    horizontal: Vector3<T>,
    /// Vertical basis vector of the viewport
    vertical: Vector3<T>
}

impl<T> Viewport<T>
where
    T: Scalar
{
    
    pub fn new_from_scalar(origin: Point3<T>, width: T, height: T) -> Self {
        let horizontal = vector![width, T::zero(), T::zero()]; 
        let vertical = vector![T::zero(), height, T::zero()];
        Self { origin, horizontal, vertical }
    }

    #[tracing::instrument]
    pub fn new_from_basis(origin: Point3<T>, horizontal: Vector3<T>, vertical: Vector3<T>) -> Self {
        Self { origin, horizontal, vertical }
    }

    /// given horizontal and vertical coordinates 0..1, returns
    /// a point if those coordinates lie within the viewport.
    pub fn point(&self, u: T, v: T) -> Option<Point3<T>> {
        let clamp = T::zero()..=T::one();
        if clamp.contains(&u) &&  clamp.contains(&v) {
            Some(self.origin + self.horizontal*u + self.vertical*v)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Camera<T>
where
    T: Scalar
{
    origin: Point3<T>,
    orientation: Unit<Vector3<T>>,
    focal_length: T,
    viewport: Viewport<T>,
    // corner: Point3<T>,
    // horizontal: Vector3<T>,
    // vertical: Vector3<T>,
}

impl<T> Camera<T>
where
     T: Scalar
{
    pub fn new_with_focus(origin: Point3<T>, look_at: Point3<T>, up: Vector3<T>) -> Self {
        let focus = look_at - origin;

        todo!();

        // Self {
        //     origin: origin,
        //     orientation: 
        // }
    }

    pub fn ray(&self, u: T, v: T) -> Option<Ray<T>> {
        // debug!{"Casting ray from camera: {:?} to ({},{})", &self, u, v};
        Some(Ray::from_focus_point(self.origin, self.viewport.point(u, v)?))
    }
}

// impl<T> Display for Camera<T>
// where
//     T: Scalar
// {
//     fn ()
// }

impl<T> Default for Camera<T>
where
    T: Scalar
{
    fn default() -> Self {
        let viewport_height = T::from_float(2.0);
        let viewport_width = viewport_height * T::from_float(AspectRatio::default().as_f64());
        let focal_length = T::from_float(1.0);

        let camera_origin = Point3::origin();
        let horizontal = vector![viewport_width, T::zero(), T::zero()];
        let vertical = vector![T::zero(), viewport_height, T::zero()];
        let depth = vector![T::zero(), T::zero(), focal_length];
        let viewport_origin = camera_origin - horizontal / T::from_float(2.0) - vertical / T::from_float(2.0) - depth;
        
        let camera = Self {
            origin: camera_origin,
            orientation: Unit::new_normalize(depth),
            focal_length: focal_length,
            viewport: Viewport::new_from_basis(viewport_origin, horizontal, vertical),
        };

        camera
    }
}


