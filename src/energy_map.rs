use image::RgbImage;

pub fn calculate_energy_map(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut output_image = RgbImage::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
       output_image.put_pixel(x, y, *pixel)
    }

    output_image
}
    