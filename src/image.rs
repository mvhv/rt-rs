pub mod aspect;
pub mod buffer;
pub mod ppm;

pub use aspect::{AspectRatio, Rectangle};
pub use buffer::PixelBuffer;
pub use ppm::AsPPM;
use tracing::debug;

use std::io;
use nalgebra::{Vector3, vector, Unit, point};
use crate::{Scalar, Camera, Material, colour};
use crate::geometry::{Sphere, Ray, Intersectable, Geometry};

#[derive(Debug)]
pub enum SphereTest {
    Flat,
    Normals,
    Shader,
}

#[tracing::instrument]
pub fn generate_sphere<T: Scalar>(shading: SphereTest, horizontal: f64, vertical: f64) -> PixelBuffer<T> {
    let mut buf = PixelBuffer::new_from_vertical_ratio(240, AspectRatio::default());
    let (height, width) = (buf.height(), buf.width());
    let camera = Camera::default();
    debug!("{:?}", camera);
    // default sphere should be red and located at (0,0,-1) just in front of the camera
    let sphere = Sphere::new(
        point![T::from_float(horizontal), T::from_float(vertical), -T::one() - T::one()],
        T::from_float(0.5),
        Material::simple_diffuse_colour(colour::green()),
    );
    debug!("{:?}", sphere);
    for row in 0..height {
        for col in 0..width {
            let u = T::from_float(col as f64 / (width - 1) as f64);
            let v = T::from_float(row as f64 / (height - 1) as f64);
            
            let ray = camera.ray(u, v).unwrap_or_else(|| {
                debug!("failed to case ray for u: {u:?} v: {v:?}");
                todo!();
            });

            buf[row * width + col] =
                if let Some(intersection) = sphere.intersect(ray) {
                    match shading {
                        SphereTest::Flat => sphere.material().colour(),
                        SphereTest::Normals => normal_to_rgb(intersection.normal()),
                        SphereTest::Shader => unimplemented!(),
                    }
                } else {
                    // otherwise we just insert the background colour
                    ray.background_colour()
                };
        }
    }

    buf
}

#[tracing::instrument]
pub fn generate_sphere_world<T: Scalar>() -> PixelBuffer<T> {
    use crate::{Material, colour, geometry::{Geometry, world::StaticWorld}};

    let mut buf = PixelBuffer::new_from_vertical_ratio(240, AspectRatio::default());
    let (height, width) = (buf.height(), buf.width());
    let camera = Camera::default();
    debug!("{:?}", camera);
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
    debug!("{:?}", world);
    for row in 0..height {
        for col in 0..width {
            let u = T::from_float(col as f64 / (width - 1) as f64);
            let v = T::from_float(row as f64 / (height - 1) as f64);
            
            let ray = camera.ray(u, v).unwrap_or_else(|| {
                debug!("failed to cast ray for u: {u:?} v: {v:?}");
                todo!();
            });

            buf[row * width + col] =
                if let Some(intersection) = world.intersect(ray) {
                    normal_to_rgb(intersection.normal())
                } else {
                    // otherwise we just insert the background colour
                    ray.background_colour()
                };
        }
    }

    buf
}

fn normal_to_rgb<T: Scalar>(normal: Unit<Vector3<T>>) -> Vector3<T> {
    (normal.into_inner() + vector![T::one(), T::one(), T::one()]) * T::from_float(0.5)
}

pub fn generate_ray_background<T: Scalar>(width: usize, height: usize) -> PixelBuffer<T> {
    let mut buf = PixelBuffer::new(width, height);
    let camera = Camera::default();

    for row in 0..(buf.height()) {
        for col in 0..(buf.width()) {
            let u = T::from_float(col as f64 / (width - 1) as f64);
            let v = T::from_float(row as f64 / (height - 1) as f64);
            buf[row * width + col] = camera
                .ray(u, v)
                .expect("Attempted to cast ray outside of viewport")
                .background_colour()
        }
    }

    buf
}

pub fn generate_gradient<T: Scalar>(width: usize, height: usize) -> PixelBuffer<T> {
    let mut pixels = vec![Vector3::zeros(); width * height];

    for row in 0..(height) {
        for col in 0..(width) {
            let red = T::from_float(col as f64 / (width - 1) as f64);
            let green = T::from_float(row as f64 / (height - 1) as f64);
            let blue = T::from_float(0.25);
            pixels[row * width + col] = vector![red, green, blue];
        }
    }

    PixelBuffer::new_from_pixels(width, height, pixels)
}


#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;
    use crate::image::AspectRatio;

    #[test]
    pub fn test_aspect_ratio() {
        let ratio = AspectRatio::new(2560, 1440);
        assert_eq!(ratio.as_tuple(), (16, 9));
    }

    #[test]
    pub fn test_gradient_f32_ppm() {
        generate_gradient::<f32>(256, 512)
            .write_ppm("test_output/gradient.ppm")
            .expect("Failed to write gradient.ppm");
    }

    #[test]
    pub fn test_ray_background_f64_ppm() {
        generate_ray_background::<f64>(400, 300)
            .write_ppm("test_output/background.ppm")
            .expect("Failed to write background.ppm")
    }

    #[test]
    pub fn test_sphere_flat_f32_ppm() {
        generate_sphere::<f32>(SphereTest::Flat, 0.0, 0.0)
            .write_ppm("test_output/sphere_flat.ppm")
            .expect("Failed to write sphere_flat.ppm")
    }

    #[test]
    pub fn test_sphere_normals_f32_ppm() {
        generate_sphere::<f64>(SphereTest::Normals, 0.0, 0.0)
            .write_ppm("test_output/sphere_normals.ppm")
            .expect("Failed to write sphere_normals.ppm")
    }

    #[test]
    pub fn test_random_f32_ppm() {
        let mut rng = rand::thread_rng();
        let pixels = (0..400*300)
            .map(|_| vector![rng.gen(), rng.gen(), rng.gen()])
            .collect::<Vec<_>>();
        PixelBuffer::<f64>::new_from_pixels(400, 300, pixels)
            .write_ppm("test_output/random.ppm")
            .expect("Failed to write random.ppm")
    }
}




