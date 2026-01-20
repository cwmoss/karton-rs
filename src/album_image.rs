/*
if ($profilename == 'big') {
            $size = '1000x800';
        } else {
            $size = '600'; //x600
        }
$mode = 'fit';
*/
use image::DynamicImage;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use image::{ColorType, GenericImageView};

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, ResizeAlg, ResizeOptions, Resizer};

use std::ops::{Div, Mul};

pub enum Sizes {
    Big,
    Small,
}

#[derive(Debug, Clone, Copy)]
pub struct Size(pub u32, pub u32);

pub fn get_size(size: Sizes) -> Size {
    match size {
        Sizes::Big => Size(1000, 800),
        _ => Size(600, 600),
    }
}

// fast version
// https://github.com/tpyo/shrinkray/blob/main/crates/lib/src/options.rs
pub fn resize_image(base: &str, album: &str, img: &str, size: Size) -> Option<DynamicImage> {
    // Placeholder implementation
    let file = format!("{}/{}/{}", base, album, img);
    let (width, height) = (size.0, size.1);
    println!(
        "Resizing image fast '{}' in album '{}' to size '{}' => {}",
        img, album, size.0, file
    );

    let img = ImageReader::open(file).unwrap().decode().unwrap();

    let (src_w, src_h) = (img.width(), img.height());

    let ar = AspectRatio::from_dimensions(img.width(), img.height());
    let target_aspect_ratio = AspectRatio::from_dimensions(width, height);

    let dest_dim = if target_aspect_ratio > ar {
        (height * ar, height)
    } else {
        (width, width / ar)
    };

    // Create container for data of destination image
    // let mut dst_image = Image::new(size.0, size.1, img.pixel_type().unwrap());
    let mut dst_image = DynamicImage::new(dest_dim.0, dest_dim.1, img.color());

    // Create Resizer instance and resize cropped source image
    // into buffer of destination image
    let mut resizer = Resizer::new();
    resizer
        .resize(
            &img,
            &mut dst_image,
            &ResizeOptions::new(), /* .crop(
                                       10.0,   // left
                                       10.0,   // top
                                       2000.0, // width
                                       2000.0, // height
                                   )*/
        )
        .unwrap();
    return Some(dst_image);
}

pub fn resize_image_img(base: &str, album: &str, img: &str, size: Size) -> Option<DynamicImage> {
    // Placeholder implementation
    let file = format!("{}/{}/{}", base, album, img);
    let (width, height) = (size.0, size.1);
    println!(
        "Resizing image '{}' in album '{}' to size '{}' => {}",
        img, album, size.0, file
    );

    // let img = image::open(&file).unwrap();
    // let img = ImageReader::open(file).ok()?.with_guessed_format().ok()?;
    let resized_img = ImageReader::open(file)
        .ok()?
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?
        .resize(width, height, image::imageops::FilterType::Lanczos3);
    // let resized_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // let resized_img = img.resize_to_fill(400, 150, image::imageops::FilterType::Lanczos3);
    return Some(resized_img);
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AspectRatio {
    pub ratio: f64,
    pub x: u32,
    pub y: u32,
}

impl AspectRatio {
    /// Create a new AspectRatio
    ///
    /// # Errors
    ///
    /// Returns an error if either x or y is zero
    pub fn new(x: u32, y: u32) -> Self {
        /*if x == 0 {
            return Err("aspect ratio numerator cannot be zero".to_string());
        }
        if y == 0 {
            return Err("aspect ratio denominator cannot be zero".to_string());
        }
        Ok(AspectRatio::from_dimensions(x, y))
        -> Result<Self, String>
        */
        AspectRatio::from_dimensions(x, y)
    }

    pub(crate) fn from_dimensions(x: u32, y: u32) -> Self {
        Self {
            ratio: f64::from(x) / f64::from(y),
            x,
            y,
        }
    }
}

impl Div<AspectRatio> for u32 {
    type Output = u32;

    fn div(self, aspect_ratio: AspectRatio) -> Self::Output {
        (f64::from(self) / aspect_ratio.ratio).round() as u32
    }
}

impl Mul<AspectRatio> for u32 {
    type Output = u32;

    fn mul(self, aspect_ratio: AspectRatio) -> Self::Output {
        (f64::from(self) * aspect_ratio.ratio).round() as u32
    }
}
