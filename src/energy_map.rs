use image::{Pixel, Rgb, RgbImage};

const DELTA_TRESHOLD: i16 = 15;

pub fn calculate_energy_map(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut output_image = RgbImage::new(width, height);

    for x in 0..(width - 1) {
        for y in 0..height {
            let current_pixel_color = image.get_pixel(x, y).0;
            let next_pixel_color = image.get_pixel(x + 1, y).0;
            let mut current_pixel_energy = 0i16;

            for channel in 0..3 {
                let current_pixel_channel = current_pixel_color[channel];
                let next_pixel_channel = next_pixel_color[channel];

                current_pixel_energy += next_pixel_channel as i16 - current_pixel_channel as i16;
            }

            let energy_representation_color = if current_pixel_energy < DELTA_TRESHOLD.into() {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            };

            output_image.put_pixel(x, y, energy_representation_color);
        }
    }

    output_image
}
