use std::io;

use crate::image::Rectangle;

pub trait AsPPM: Rectangle {
    fn ppm_body(&self) -> String;
    fn write_ppm(&self, filename: &str) -> io::Result<()>;

    fn as_ppm_string(&self) -> String {
        // "P3" (ASCII encoding)
        // int int (width, height)
        // int (max colour value)
        // int int int ... (RGB triplets of pixel colours)
        format!("P3\n{} {}\n255\n{}", Rectangle::width(self), Rectangle::height(self), self.ppm_body())
    }
}