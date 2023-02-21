pub mod aspect;
pub mod buffer;
pub mod ppm;

pub use aspect::{AspectRatio, Rectangle};
pub use buffer::PixelBuffer;
use nalgebra::Vector3;
pub use ppm::AsPPM;
use tracing::{debug, trace};

use rayon::prelude::*;
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};
use crate::{Scalar, Camera, colour};
use crate::geometry::Intersectable;

const SAMPLES: usize = 100;
const BOUNCES: usize = 50;

#[derive(Debug)]
pub enum SphereTest {
    Flat,
    Normals,
    Shader,
}

// pub fn generate_sphere<T: Scalar>(shading: SphereTest, horizontal: f64, vertical: f64) -> PixelBuffer<T> {
//     let mut buf = PixelBuffer::new_from_vertical_ratio(240, AspectRatio::default());
//     let (height, width) = (buf.height(), buf.width());
//     let camera = Camera::default();
//     debug!("{:?}", camera);
//     // default sphere should be red and located at (0,0,-1) just in front of the camera
//     let sphere = Sphere::new(
//         point![T::from_float(horizontal), T::from_float(vertical), -T::one() - T::one()],
//         T::from_float(0.5),
//         Material::simple_diffuse_colour(colour::green()),
//     );
//     debug!("{:?}", sphere);
//     for row in 0..height {
//         for col in 0..width {
//             let u = T::from_float(col as f64 / (width - 1) as f64);
//             let v = T::from_float(row as f64 / (height - 1) as f64);
            
//             let ray = camera.ray(u, v).unwrap_or_else(|| {
//                 debug!("failed to cast ray for u: {u:?} v: {v:?}");
//                 todo!();
//             });

//             buf[row * width + col] =
//                 if let Some(intersection) = sphere.intersect(ray, T::from_float(0.1), T::from_float(100.0)) {
//                     match shading {
//                         SphereTest::Flat => sphere.material().colour(),
//                         SphereTest::Normals => normal_to_rgb(intersection.normal()),
//                         SphereTest::Shader => unimplemented!(),
//                     }
//                 } else {
//                     // otherwise we just insert the background colour
//                     ray.background_colour()
//                 };
//         }
//     }

//     buf
// }

fn make_render_progress_bar(width: usize, height: usize) -> ProgressBar {
    ProgressBar::new((height * width) as u64)
        .with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} ({percent}%) [{eta}] {msg}")
                .expect("Failed to create render progress bar")
                .progress_chars("##-")
        )
}

/// applies jitter [0.0, 1.0) to the input and scales by the given factor
fn jitter_scale<T: Scalar>(value: usize, scale: T) -> T {
    T::from_float(value as f64 + rand::thread_rng().gen_range(0.0..1.0_f64)) * scale
}

pub fn render_scene<T: Scalar, Scene: Intersectable<T>>(camera: Camera<T>, scene: Scene) -> PixelBuffer<T> {
    let mut buf = PixelBuffer::new_from_vertical_ratio(720, AspectRatio::default());
    let (width, height) = Rectangle::as_tuple(&buf);
    // scale factors for converting pixel indicies to camera coordinates
    let width_scale = T::one() / T::from_float(width as f64);
    let height_scale = T::one() / T::from_float(height as f64);

    let render_progress = make_render_progress_bar(width, height);
    for row in 0..height {
        for col in 0..width {
            render_progress.inc(1);

            let mut sample_acc = colour::black();
            for _ in 0..SAMPLES {
                let u = jitter_scale(col, width_scale);
                let v = jitter_scale(row, height_scale);
                trace!("Sampling pixel: (col: {col:?}, row: {row:?}), coord: (u: {u:?} v: {v:?}), rect: (width: {width:?}, height: {height:?})");

                let mut ray = camera.ray(u, v)
                    .expect("camera coordinates should be in range [0,1)");
                for _ in 0..BOUNCES {
                    if let Some(intersection) = scene.intersect(ray, T::from_float(0.00001), T::INF) {
                        ray = intersection.scatter();
                        trace!("Scatter: {ray:?}");  
                    } else {
                        sample_acc += ray.colour();
                        break;
                    }
                }
            }
            buf[row * width + col] = sample_acc / T::from_float(SAMPLES as f64);
        }
    }

    buf
}



pub fn render_scene_parallel<T, Scene>(camera: Camera<T>, scene: Scene, vertical_resolution: usize) -> PixelBuffer<T> 
where
    T: Scalar,
    Scene: Intersectable<T> + Sync
{
    render_scene_parallel_quality(camera, scene, vertical_resolution, SAMPLES)
}

pub fn render_scene_parallel_quality<T, Scene>(camera: Camera<T>, scene: Scene, vertical_resolution: usize, samples: usize) -> PixelBuffer<T> 
where
    T: Scalar,
    Scene: Intersectable<T> + Sync
{
    let mut buf = PixelBuffer::new_from_vertical_ratio(vertical_resolution, AspectRatio::default());
    let (width, height) = Rectangle::as_tuple(&buf);
    // scale factors for converting pixel indicies to camera coordinates
    let width_scale = T::one() / T::from_float(width as f64);
    let height_scale = T::one() / T::from_float(height as f64);

    let render_progress = make_render_progress_bar(width, height);
    buf.pixels = (0..height).into_par_iter().flat_map(|row| {
        let mut row_buf = vec![Vector3::zeros(); width];
        for col in 0..width {
            render_progress.inc(1);

            let mut sample_acc = colour::black();
            for _ in 0..samples {
                let u = jitter_scale(col, width_scale);
                let v = jitter_scale(row, height_scale);
                trace!("Sampling pixel: (col: {col:?}, row: {row:?}), coord: (u: {u:?} v: {v:?}), rect: (width: {width:?}, height: {height:?})");

                let mut ray = camera.ray(u, v)
                    .expect("camera coordinates should be in range [0,1)");
                for _ in 0..BOUNCES {
                    if let Some(intersection) = scene.intersect(ray, T::from_float(0.00001), T::INF) {
                        ray = intersection.scatter();
                        trace!("Scatter: {ray:?}");  
                    } else {
                        sample_acc += ray.colour();
                        break;
                    }
                }
            }
            row_buf[col] = sample_acc / T::from_float(samples as f64);
        }
        row_buf.into_par_iter()
    })
    .collect();

    buf
}

// pub fn render_scene<T: Scalar, Scene: Intersectable<T>>(camera: Camera<T>, scene: Scene) -> PixelBuffer<T> {
//     let mut rng = rand::thread_rng();
//     let mut buf = PixelBuffer::new_from_vertical_ratio(180, AspectRatio::default());
//     let (width, height) = Rectangle::as_tuple(&buf);

//     let render_progress = make_render_progress_bar(width, height);
//     for row in 0..height {
//         for col in 0..width {
//             render_progress.inc(1);
//             let mut sample_acc = colour::black();//vector![0.0_f64, 0.0, 0.0];
//             for _ in 0..SAMPLES {
//                 let u = T::from_float((col as f64 + rng.gen_range(0.0..1.0_f64)) / width as f64);
//                 let v = T::from_float((row as f64 + rng.gen_range(0.0..1.0_f64)) / height as f64);
               
//                 trace!("Sampling pixel: (col: {col:?}, row: {row:?}), coord: (u: {u:?} v: {v:?}), rect: (width: {width:?}, height: {height:?})");                
//                 if let Some(ray) = camera.ray(u, v) {
//                     sample_acc += if let Some(first_intersect) = scene.intersect(ray, T::from_float(0.01), T::from_float(100.0)) {
//                         let mut bounce_acc = first_intersect.material().colour() * T::HALF;
//                         let mut last_intersection = first_intersect;
//                         for bounce in 1..BOUNCES {
//                             let curr_ray = last_intersection.random_diffuse_ray();
//                             let contribution = T::from_float(0.5_f64.powi(bounce + 1));
//                             trace!("Diffuse bounce: {curr_ray:?}");
//                             if let Some(curr_intersection) = scene.intersect(curr_ray, T::from_float(0.01), T::from_float(100.0)) {
//                                 last_intersection = curr_intersection;
//                                 bounce_acc += last_intersection.material().colour() * contribution;
//                             } else {
//                                 // colour::black()
//                                 bounce_acc += ray.background_colour() * contribution;
//                                 break; // if nothing hit, then stop trying to bounce
//                             }
//                         }
//                         bounce_acc
//                     } else {
//                         // otherwise we just insert the background colour
//                         ray.background_colour()
//                     };
//                 } else {
//                     warn!("failed to cast ray for subsample: (u: {u:?} v: {v:?}), pixel (col: {col:?}, row: {row:?}) on rect: (width: {width:?}, height: {height:?})");
//                     todo!();
//                 }
//             }
//             buf[row * width + col] = sample_acc / T::from_float(SAMPLES as f64);
//         }
//     }

//     buf
// }

// pub fn generate_aa_world<T: Scalar>() -> PixelBuffer<T> {
//     // use crate::{colour, geometry};
//     // use rand::Rng;
//     // let mut rng = rand::thread_rng();

//     // let mut buf = PixelBuffer::new_from_vertical_ratio(180, AspectRatio::default());
//     // let (width, height) = Rectangle::as_tuple(&buf);
//     let camera = Camera::default();
//     // debug!("{camera:?}");
//     let scene = world::one_sphere_scene();
    
//     render_scene(camera, scene)
// }

// pub fn generate_sphere_world<T: Scalar>() -> PixelBuffer<T> {
//     use crate::{Material, colour, geometry::{Geometry, world::StaticWorld}};

//     let mut buf = PixelBuffer::new_from_vertical_ratio(240, AspectRatio::default());
//     let (height, width) = (buf.height(), buf.width());
//     let camera = Camera::default();
//     debug!("{:?}", camera);
//     let world = world::three_sphere_scene();
//     debug!("{:?}", world);
//     for row in 0..height {
//         for col in 0..width {
//             let u = T::from_float(col as f64 / (width - 1) as f64);
//             let v = T::from_float(row as f64 / (height - 1) as f64);
            
//             let ray = camera.ray(u, v).unwrap_or_else(|| {
//                 debug!("failed to cast ray for u: {u:?} v: {v:?}");
//                 todo!();
//             });

//             buf[row * width + col] =
//                 if let Some(intersection) = world.intersect(ray, T::from_float(0.1), T::from_float(100.0)) {
//                     normal_to_rgb(intersection.normal())
//                 } else {
//                     // otherwise we just insert the background colour
//                     ray.background_colour()
//                 };
//         }
//     }

//     buf
// }


#[cfg(test)]
mod test {
    use rand::Rng;
    use nalgebra::{Vector3, vector, Unit};

    use super::*;
    use crate::image::AspectRatio;
    
    // fn normal_to_rgb<T: Scalar>(normal: Unit<Vector3<T>>) -> Vector3<T> {
    //     (normal.into_inner() + vector![T::one(), T::one(), T::one()]) * T::from_float(0.5)
    // }

    fn generate_ray_background<T: Scalar>(width: usize, height: usize) -> PixelBuffer<T> {
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
    
    fn generate_gradient<T: Scalar>(width: usize, height: usize) -> PixelBuffer<T> {
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

    // #[test]
    // pub fn test_sphere_flat_f32_ppm() {
    //     generate_sphere::<f32>(SphereTest::Flat, 0.0, 0.0)
    //         .write_ppm("test_output/sphere_flat.ppm")
    //         .expect("Failed to write sphere_flat.ppm")
    // }

    // #[test]
    // pub fn test_sphere_normals_f32_ppm() {
    //     generate_sphere::<f64>(SphereTest::Normals, 0.0, 0.0)
    //         .write_ppm("test_output/sphere_normals.ppm")
    //         .expect("Failed to write sphere_normals.ppm")
    // }

    // #[test]
    // pub fn test_sphere_aa_f64_ppm() {
    //     generate_aa_world::<f64>()
    //         .write_ppm("test_output/sphere_aa_f64.ppm")
    //         .expect("Failed to write sphere_aa_f64.ppm")
    // }
    
    // #[test]
    // pub fn test_sphere_aa_f32_ppm() {
    //     generate_aa_world::<f32>()
    //         .write_ppm("test_output/sphere_aa_f32.ppm")
    //         .expect("Failed to write sphere_aa_f32.ppm")
    // }

    // #[test]
    // pub fn test_sphere_gamma_double_f32_ppm() {
    //     generate_aa_world::<f32>()
    //         .apply_gamma(2.0)
    //         .write_ppm("test_output/sphere_gamma)double_f32.ppm")
    //         .expect("Failed to write sphere_gamma_double_f32.ppm")
    // }

    // #[test]
    // pub fn test_sphere_gamma_half_f32_ppm() {
    //     generate_aa_world::<f32>()
    //         .apply_gamma(0.5)
    //         .write_ppm("test_output/sphere_gamma_half_f32.ppm")
    //         .expect("Failed to write sphere_gamma__half_f32.ppm")
    // }

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




