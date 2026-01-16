/*
if ($profilename == 'big') {
            $size = '1000x800';
        } else {
            $size = '600'; //x600
        }
$mode = 'fit';
*/
use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::ImageReader;
pub enum Sizes {
    Big,
    Small,
}
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
