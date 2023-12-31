use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::{color::Color, interval::Interval, rtw_image::RtwImage, vec3::Point3};

#[enum_dispatch]
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[enum_dispatch(Texture)]
#[derive(Clone)]
pub enum AnyTexture {
    SolidColor,
    CheckerTexture,
    ImageTexture,
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }

    // pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
    //     Self::new(Color::new(red, green, blue))
    // }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<AnyTexture>,
    odd: Arc<AnyTexture>,
}

impl CheckerTexture {
    // pub fn new(scale: f64, even: impl Into<AnyTexture>, odd: impl Into<AnyTexture>) -> Self {
    //     Self {
    //         inv_scale: 1. / scale,
    //         even: Arc::new(even.into()),
    //         odd: Arc::new(odd.into()),
    //     }
    // }

    pub fn from_solid(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: Arc::new(SolidColor::new(c1).into()),
            odd: Arc::new(SolidColor::new(c2).into()),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0., 1.).clamp(u);
        // Flip V to image coordinates
        let v = 1.0 - Interval::new(0., 1.).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
