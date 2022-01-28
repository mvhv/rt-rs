#[derive(Debug, PartialEq, Eq)]
pub struct AspectRatio {
    horizontal: usize,
    vertical: usize,
}

impl AspectRatio {
    pub fn new(width: usize, height: usize) -> Self {
        let gcd = num::integer::gcd(width, height);
        Self {
            horizontal: width / gcd,
            vertical: height / gcd,
        }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.horizontal, self.vertical)
    }

    pub fn as_f64(&self) -> f64 {
        self.horizontal as f64 / self.vertical as f64
    }

    pub fn as_f32(&self) -> f32 {
        self.horizontal as f32 / self.vertical as f32
    }

    pub fn vertical(&self) -> usize {
        self.vertical
    }

    pub fn horizontal(&self) -> usize {
        self.horizontal
    }

    pub fn height_from_width(&self, width: usize) -> usize {
        (width as f64 / self.as_f64()) as usize
    }

    pub fn width_from_height(&self, height: usize) -> usize {
        (height as f64 * self.as_f64()) as usize
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self {
            horizontal: 16,
            vertical: 9
        }
    }
}

impl From<(usize, usize)> for AspectRatio {
    fn from(val: (usize, usize)) -> Self {
        let (h, v) = val;
        Self::new(h, v)
    }
}

pub trait Rectangle {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}