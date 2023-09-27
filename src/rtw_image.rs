use std::{process, sync::Arc};

use image::io::Reader as ImageReader;

const BYTES_PER_PIXEL: i32 = 3;

#[derive(Clone)]
pub struct RtwImage {
    data: Arc<[u8]>,
    image_width: i32,
    image_height: i32,
    bytes_per_scanline: i32,
}

impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let mut image_width = 0;
        let mut image_height = 0;

        let data = (|| {
            let image = ImageReader::open(filename).ok()?.decode().ok()?.into_rgb8();
            image_width = image.width() as i32;
            image_height = image.height() as i32;
            Some(image.into_vec().into())
        })();

        let Some(data) = data else {
            eprintln!("ERROR: Could not load image file {filename}.");
            process::exit(1)
        };

        Self {
            data,
            image_width,
            image_height,
            bytes_per_scanline: image_width * BYTES_PER_PIXEL,
        }
    }

    pub fn width(&self) -> i32 {
        self.image_width
    }

    pub fn height(&self) -> i32 {
        self.image_height
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        // Return the address of the three bytes of the pixel at x,y (or magenta if no data).
        let x = Self::clamp(x, 0, self.image_width);
        let y = Self::clamp(y, 0, self.image_height);

        self.data[(y * self.bytes_per_scanline + x * BYTES_PER_PIXEL) as usize..][..3]
            .try_into()
            .unwrap()
    }

    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        // Return the value clamped to the range [low, high).
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}
