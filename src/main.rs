use std::io;

use nalgebra::{vector, Point3, Vector3};
use tracing::{debug, error, info, span, warn, Level};

use raytracing::{
    image::{
        PixelBuffer,
        AspectRatio,
        Rectangle,
        AsPPM
    },
    geometry::Ray,
    camera::Camera,
};

// #[tracing::instrument]
fn main() -> io::Result<()> {    

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .pretty()
        .init();

    // simple test
    // PixelBuffer::<f32>::new_from_vertical_ratio(1080, (16, 9).into())
    //     .write_ppm("test_output/blank1080.ppm")?;
    debug!("setting up sphere");
    let img = raytracing::image::generate_sphere::<f32>(
        raytracing::image::SphereTest::Normals,
        0.0,
        0.0
    );
    debug!("sphere created");
    img.write_ppm("asdf.ppm")?;
    debug!("sphere written to asdf.ppm");

    debug!("setting up world");
    let img = raytracing::image::generate_sphere_world::<f32>();
    debug!("sphere world created");
    img.write_ppm("sphere_world.ppm")?;
    debug!("sphere written to sphere_world.ppm");
        
    // let mut PixelBuffer::<f64>::new_from_horizontal_ratio(600, (600, 300).into());
    Ok(())
}


