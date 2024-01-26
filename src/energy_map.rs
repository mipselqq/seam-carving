use image::{Pixel, Rgb, RgbImage};

pub fn calculate_energy_map(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut output_image = RgbImage::new(width, height);

    for x in 0..(width - 1) {
        for y in 0..height {
            let current_pixel_color = image.get_pixel(x, y).0;
            let next_pixel_color = image.get_pixel(x + 1, y).0;
            let mut current_pixel_energy = 0i16;

            // TODO: use more neighbours (including y axis) to calculate the enery
            // or just find a better formula like sobel filter
            for channel in 0..3 {
                let current_pixel_channel = current_pixel_color[channel];
                let next_pixel_channel = next_pixel_color[channel];

                current_pixel_energy += (next_pixel_channel as i16 - current_pixel_channel as i16).abs();
            }

            let energy_representation_color = Rgb([
                current_pixel_energy as u8,
                current_pixel_energy as u8,
                current_pixel_energy as u8,
            ]);

            output_image.put_pixel(x, y, energy_representation_color);
        }
    }

    output_image
}
