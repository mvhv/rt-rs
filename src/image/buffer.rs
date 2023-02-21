use std::ops::{Index, IndexMut};
use std::{fs, io, path};
use std::io::Write;

use nalgebra::{Vector3, vector};
use tracing::debug;

use crate::Scalar;
use crate::image::{AspectRatio, Rectangle, AsPPM};

#[derive(Debug)]
pub struct PixelBuffer<T>
where
    T: Scalar
{
    width: usize,
    height: usize,
    aspect: AspectRatio,
    pub pixels: Vec<Vector3<T>>
}

impl<T> PixelBuffer<T>
where
    T: Scalar
{
    pub fn new(width: usize, height: usize) -> Self {
        let aspect = AspectRatio::new(width, height);
        let pixels = vec![Vector3::zeros(); width * height];
        Self { width, height, aspect, pixels }
    }
    
    pub fn new_from_pixels(width: usize, height: usize, pixels: Vec<Vector3<T>>) -> Self {
        let aspect = AspectRatio::new(width, height);
        Self { width, height, aspect, pixels }
    }

    pub fn new_from_horizontal_ratio(width: usize, aspect: AspectRatio) -> Self {
        let height = aspect.height_from_width(width);
        let pixels = vec![Vector3::zeros(); width * height];
        Self { width, height, aspect, pixels }
    }

    pub fn new_from_vertical_ratio(height: usize, aspect: AspectRatio) -> Self {
        let width = aspect.width_from_height(height);
        let pixels = vec![Vector3::zeros(); width * height];
        Self { width, height, aspect, pixels }
    }

    pub fn aspect_ratio(&self) -> &AspectRatio {
        &self.aspect
    }

    pub fn apply_gamma(self, gamma: T) -> Self {
        let pixels = self.pixels
            .into_iter()
            .map(|px| {
                px.map(|ch| {
                    ch.powf(gamma)
                })
            }).collect();
        Self { pixels, ..self }
    }

    pub fn as_u32(&self) -> Vec<u32> {
        self.pixels
            .iter()
            .map(|px| {
                let px_u8 = px.map(|ch| ch.scale_to_u8());
                from_u8_rgb(px_u8.x, px_u8.y, px_u8.z)
            }).collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

impl<T> Rectangle for PixelBuffer<T>
where
    T: Scalar
{
    fn width(&self) -> usize {
        self.width
    }
    
    fn height(&self) -> usize {
        self.height
    }
}

// moved gamma scaling to optional method on PixelBuffer
// fn pixel_as_u8_gamma<T: Scalar>(pixel: Vector3<T>, gamma: T) -> Vector3<u8> {
//     pixel.map(|ch| ch.powf(gamma).scale_to_u8())
// }

fn pixel_as_u8<T: Scalar>(pixel: Vector3<T>) -> Vector3<u8> {
    pixel.map(|ch| ch.scale_to_u8())
}

impl<T> AsPPM for PixelBuffer<T>
where
    T: Scalar
{
    fn ppm_body(&self) -> String {
        let mut buffer = String::new();
        for row in (0..(self.height)).rev() { // range reversed to write from bottom left upwards
            for col in 0..(self.width) {
                let pixel = pixel_as_u8(self.pixels[row * self.width + col]);
                buffer.push_str(
                    &format!("{} {} {} ", pixel[0], pixel[1], pixel[2])
                );
            }
            buffer.push_str("\n");
        }
        buffer
    }

    fn write_ppm(&self, filename: &str) -> io::Result<()> {
        let path = path::Path::new(filename);
        if let Some(parent) = path.parent() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(parent)?;
        };
    
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        
        debug!("writing framebuffer to {filename}");

        file.write_all(
            self.as_ppm_string()
                .as_bytes()
        )
    }
}

impl<T> Index<usize> for PixelBuffer<T>
where
    T: Scalar
{
    type Output = Vector3<T>;
    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
        &self.pixels[i]
    }
}

impl<T> IndexMut<usize> for PixelBuffer<T>
where
    T: Scalar
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        &mut self.pixels[i]
    }
}