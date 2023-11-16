use core::fmt::Debug;

use nalgebra::{vector, Vector3, Point3, Unit};
use rand::Rng;
use rand_distr::StandardNormal;

use crate::{
    Scalar,
    material::{Material, self},
    geometry::{Ray, Aabb},
    colour
};


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Face {
    Front,
    Back,
}

#[derive(Clone, Copy, Debug)]
enum Scatter {
    Diffuse,
    Specular,
    Refract,
}

#[derive(Debug)]
pub struct Intersection<T>
where
    T: Scalar
{
    point: Point3<T>,
    normal: Unit<Vector3<T>>,
    material: Material<T>,
    face: Face,
    incident: Ray<T>,
}

enum Refract {
    Refract,
    Reflect,
}

/// return an enum indicating if this refraction is possible or if internal reflection occurs
fn refraction_type<T: Scalar>(incident: Unit<Vector3<T>>, normal: Unit<Vector3<T>>, refraction_ratio: T) -> Refract {
    let cos_theta = incident.dot(&normal);
    let sin_theta = (T::one() - cos_theta * cos_theta).sqrt();
    if (refraction_ratio * sin_theta > T::one()) {
        Refract::Reflect
    } else {
        Refract::Refract
    }
}

impl<T> Intersection<T>
where
    T: Scalar
{
    pub fn new(point: Point3<T>, incident: Ray<T>, normal: Unit<Vector3<T>>, material: Material<T>) -> Self {
        let face = if normal.dot(&incident.orientation()) > T::zero() { Face::Back } else { Face::Front };
        Intersection { point, normal, material, face, incident }
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

    // pub fn front_face(&self) -> bool {
    //     self.face == Face::Front
    // }

    // pub fn back_face(&self) -> bool {
    //     self.face == Face::Back
    // }
  
    /// select an orientation for this
    fn scatter_type(&self) -> Scatter {
        let roll_transmit = T::from_float(rand::thread_rng().gen());
        if roll_transmit < self.material.transmissibility() {
            return Scatter::Refract;
        };

        let roll_reflect = T::from_float(rand::thread_rng().gen());
        if roll_reflect < self.material.coherency() {
            Scatter::Specular
        } else {
            Scatter::Diffuse
        }
    }

    /// return an orientation for a diffuse reflection from this intersection
    fn lambertian_orientation(&self) -> Vector3<T> {
        self.normal.into_inner() + random_spherical_unit().into_inner()
    }

    /// return an orientation for a specular reflection from this intersection
    fn specular_orientation(&self) -> Vector3<T> {
        // for ray `v` incident on surface with normal `n` reflected ray = v - 2 * (v . n) * n
        let v = self.incident.orientation().into_inner();
        let n = self.normal().into_inner();
        v - n * v.dot(&n) * T::TWO
        // self.incident.orientation().into_inner() - &self.normal * self.incident.orientation().dot(&self.normal)(&self.normal) * T::from_float(2.0)
    }

    /// return an orientation for a refraction from this intersection
    fn refracted_orientation(&self) -> (Vector3<T>, T) {
        // these calculations appropriated from https://raytracing.github.io/books/RayTracingInOneWeekend.html
        // snells law eta * sin(theta) = eta' * sin(theta')
        // for unit incident ray R and unit refracted ray R'
        // decompose R' = R'(per) + R'(par)
        // R'(per) = (eta / eta') * (R + cos(theta) * n)
        // R'(par) = -sqrt(1 - |R'(per)|^2) * n
        // dot product can be expressed in terms of magnitude and cos(theta)
        // a . b = |a|*|b|*cos(theta)
        // and for a and b unit vectors
        // a . b = cos(theta)
        // therefore
        // R'(per) = (eta / eta') * (R + (-R . n) * n)
        
        let ray_in = self.incident.orientation();
        let (eta_in, eta_out, normal_in) = match self.face {
            Face::Front => (self.incident.medium(), self.material.refractive_index(), self.normal),
            // Face::Back => (self.material.refractive_index(), self.incident.medium(), -self.normal)
            // assuming the back face exits into air, this may not always be true so should improve the incident ray code
            Face::Back => (self.material.refractive_index(), T::from_float(material::refractive_index::AIR), -self.normal)
        };
        let refraction_ratio = eta_in / eta_out;
    
        // first check if refraction or reflection
        match refraction_type(self.incident.orientation(), normal_in, refraction_ratio) {
            Refract::Refract => {
                let ray_out_perpendicular = (normal_in.into_inner() * (-ray_in).dot(&normal_in) + ray_in.into_inner()) * refraction_ratio;
                let ray_out_parallel = normal_in.into_inner() * (-(T::one() - ray_out_perpendicular.norm_squared()).sqrt());
                (ray_out_parallel + ray_out_perpendicular, eta_out)
            },
            Refract::Reflect => {
                (self.specular_orientation(), eta_in)
            }
        }


    }

    // generate a scattered ray based on material properties
    pub fn scatter(&self) -> Ray<T> {
        // we're assuming the normal is still normalised here
        let (orientation, medium) = match self.scatter_type() {
            Scatter::Diffuse => (self.lambertian_orientation(), self.incident.medium()),
            Scatter::Specular => (self.specular_orientation(), self.incident.medium()),
            Scatter::Refract => self.refracted_orientation(),
        };
        // TODO: Remove this checkerboard hack once proper textures are implemented
        let albedo_scale = if self.material.checkerboard && checkerboard_dark(self.point) {
            T::from_float(0.1)
        } else {
            T::one()
        };
        let albedo = self.material.albedo() * albedo_scale;
        let attenuation = colour::white() - self.incident.gain().component_mul(&albedo);
        Ray::from_orientation_attenuated(self.point, orientation, attenuation).with_medium(medium)
    }
}

/// simple hacky euclidean remainder
fn modulo<T: Scalar>(a: T, b: T) -> T {
    ((a % b) + b) % b
}

/// checks if a point should be darkened as part of the checkerboard
fn checkerboard_dark<T: Scalar>(point: Point3<T>) -> bool {
    // point.map(|dim| dim.rem(T::TWO) >= T::one()).any
    // (point.x.rem(T::TWO) >= T::one() or point.y.rem(T::TWO)
    (modulo(point.x, T::TWO) >= T::one()) ^ (modulo(point.z, T::TWO) >= T::one())
}

/// returns a random vector on uniformly distributed over the surface of the unit sphere
/// generate normally distributed values for x, y, z and normalise
fn random_spherical_unit<T: Scalar>() -> Unit<Vector3<T>> {
    let mut rng = rand::thread_rng();
    Unit::new_normalize(vector![
        T::from_float(rng.sample(StandardNormal)),
        T::from_float(rng.sample(StandardNormal)),
        T::from_float(rng.sample(StandardNormal))
    ])
}

pub trait Intersectable<T>
where
    T: Scalar + Debug
{
    fn intersect(&self, ray: Ray<T>, min_depth: T, max_depth: T) -> Option<Intersection<T>>;
    fn bounding_box(&self) -> Option<Aabb<T>>;
    // fn material(&self) -> Material<T>;
    // fn normal(&self, point: Point3<T>) -> Unit<Vector3<T>>;
}