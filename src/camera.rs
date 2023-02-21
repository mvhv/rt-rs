use nalgebra::{Point3, Vector3, Unit, vector, point};
use rand::Rng;
use rand_distr::StandardNormal;
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
    u: Unit<Vector3<T>>,
    v: Unit<Vector3<T>>,
    w: Unit<Vector3<T>>,
    orientation: Unit<Vector3<T>>,
    focal_length: T,
    viewport: Viewport<T>,
    // defocus disk size
    aperture: T,
    // corner: Point3<T>,
    // horizontal: Vector3<T>,
    // vertical: Vector3<T>,
}

impl<T> Camera<T>
where
     T: Scalar
{
    // pub fn new_with_focus(origin: Point3<T>, look_at: Point3<T>, up: Vector3<T>) -> Self {
    //     let focus = look_at - origin;

    //     todo!();

    //     // Self {
    //     //     origin: origin,
    //     //     orientation: 
    //     // }
    // }


    fn ray_source(&self) -> Point3<T> {
        let mut rng = rand::thread_rng();
        let (delta_u, delta_v) = (T::from_float(rng.sample(StandardNormal)), T::from_float(rng.sample(StandardNormal)));
        let defocus = (self.u.into_inner() * delta_u + self.v.into_inner() * delta_v) * self.aperture;
        self.origin + defocus
    }

    pub fn ray(&self, u: T, v: T) -> Option<Ray<T>> {
        Some(Ray::from_focus_point(self.ray_source(), self.viewport.point(u, v)?))
    }

    pub fn with_aperture(self, aperture: T) -> Camera<T> {
        Self { aperture, ..self }
    }

    // pub fn from_vfov(vertical_fov: T, aspect_ratio: AspectRatio) -> Camera<T> {
    //     let origin = Point3::origin();
    //     let theta = degrees_to_radians(vertical_fov);
    //     let h = (theta / T::TWO).tan();

    //     let viewport_height = T::TWO * h;
    //     let viewport_width = viewport_height * T::from_float(aspect_ratio.as_f64());

    //     let focus = point![T::zero(), T::zero(), -T::one()];
    //     let focal_length = (focus - origin).norm();

    //     let horizontal = vector![viewport_width, T::zero(), T::zero()];
    //     let vertical = vector![T::zero(), viewport_height, T::zero()];
        
    //     let depth = vector![T::zero(), T::zero(), focal_length];
        
    //     let viewport_origin = origin - horizontal / T::TWO - vertical / T::TWO - depth;
    
    //     Self {
    //         origin: origin,
    //         orientation: Unit::new_normalize(depth),
    //         focal_length: focal_length,
    //         viewport: Viewport::new_from_basis(viewport_origin, horizontal, vertical),
    //         aperture: 0
    //     }


    // }

    pub fn look_at(origin: Point3<T>, focus: Point3<T>, vertical_fov: T, aspect_ratio: AspectRatio) -> Camera<T> {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0.into()).tan();
        let viewport_height = T::TWO * h;
        let viewport_width = viewport_height * T::from_float(aspect_ratio.as_f64());

        // positive w is behind the camera
        let w = Unit::new_normalize(origin - focus);
        let u = Unit::new_normalize(vup().cross(&w));
        let v = Unit::new_normalize(w.cross(&u));

        let horizontal = u.into_inner() * viewport_width;
        let vertical = v.into_inner() * viewport_height;
        let viewport_origin = origin - horizontal / T::TWO - vertical / T::TWO - w.into_inner();
    
        Self {
            origin, u, v, w,
            orientation: -w,
            focal_length: w.norm(),
            viewport: Viewport::new_from_basis(viewport_origin, horizontal, vertical),
            aperture: T::zero(),
        }
    }
}

fn degrees_to_radians<T: Scalar>(n: T) -> T {
    n * T::pi() / T::from_float(180.0)
}

/// return the "view-up" vector, +y in the universal reference frame
fn vup<T: Scalar>() -> Unit<Vector3<T>> {
    Vector3::y_axis()
    // Unit::new_normalize(vector![T::zero(), T::one(), T::zero()])
}

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
        
        let u = Unit::new_normalize(horizontal);
        let v = Unit::new_normalize(vertical);
        let w = Unit::new_normalize(depth);

        Self {
            origin: camera_origin,
            u, v, w,
            orientation: -w,
            focal_length,
            viewport: Viewport::new_from_basis(viewport_origin, horizontal, vertical),
            aperture: T::zero()
        }
    }
}


