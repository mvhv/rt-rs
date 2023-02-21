use std::io;

use nalgebra::point;
// use nalgebra::{vector, Point3, Vector3};
use tracing::{debug, error, info, span, warn, Level};

use raytracing::{
    image::{
        AsPPM,
        render_scene_parallel_quality,
        AspectRatio,
    },
    geometry::world,
    camera::Camera,
};

#[tracing::instrument]
fn main() -> io::Result<()> {    

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // .with_ansi(false)
        .pretty()
        .init();

    debug!("setting up scene");
    // let camera = Camera::<f32>::default();
    let origin = point![0.5, -0.3, 0.0];
    let focus = point![0.1, -0.1, -1.0];
    let camera = Camera::look_at(origin, focus, 80.0_f32, AspectRatio::default());//.with_aperture(0.0075);
    // let camera = Camera::default();
    let scene = world::ten_sphere_scene();
    debug!("rendering scene");
    render_scene_parallel_quality(camera, scene, 720, 100)
        .apply_gamma(0.5)
        .write_ppm("test_output/80vfov_sphere_scene_dof.ppm")?;
    // if let Ok(mut window) = minifb::Window::new(
    //     "Test",
    //     640,
    //     400,
    //     minifb::WindowOptions {
    //         // resize: true,
    //         // scale: true,
    //         ..minifb::WindowOptions::default()
    //     }
    // ) {
    //     window.update_with_buffer(
    //         &buf.as_u32(),
    //         buf.width(),
    //         buf.height()
    //     ).unwrap();
    //     std::thread::sleep(std::time::Duration::new(10, 0));
    // } else {
    //     panic!("unable to create window");
    // }


    Ok(())
}


