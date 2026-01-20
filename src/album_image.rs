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

pub fn resize_image(base: &str, album: &str, img: &str, size: Size) -> Option<DynamicImage> {
    // Placeholder implementation
    let file = format!("{}/{}/{}", base, album, img);
    let (width, height) = (size.0, size.1);
    println!(
        "Resizing image fast '{}' in album '{}' to size '{}' => {}",
        img, album, size.0, file
    );

    let img = ImageReader::open(file).unwrap().decode().unwrap();

    // Create container for data of destination image
    // let mut dst_image = Image::new(size.0, size.1, img.pixel_type().unwrap());
    let mut dst_image = DynamicImage::new(size.0, size.1, img.color());

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
