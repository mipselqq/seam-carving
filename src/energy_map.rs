use image::{Pixel, RgbImage};
use crate::matrix::Matrix;

pub fn calculate_energy_map(image: &RgbImage) -> Matrix<u8> {
    let (width, height) = image.dimensions();
    let mut energy_map = Matrix::new(height, width, vec![0; (height * width) as usize]);

    let sobel_coefficients = [-1, 0, 1];

    // This saves a couple of milliseconds
    let image_buffer: Vec<_> = image.pixels().collect();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut gradient_x = 0i16;
            let mut gradient_y = 0i16;

            for x_offset in 0..3 {
                for y_offset in 0..3 {
                    let pixel_intensity = image_buffer[(x + x_offset - 1 + (y + y_offset - 1) * width) as usize].to_luma()[0] as i16;

                    gradient_x += sobel_coefficients[x_offset as usize] * pixel_intensity;
                    gradient_y += sobel_coefficients[y_offset as usize] * pixel_intensity;
                }
            }

            let energy = (gradient_x.abs().max(gradient_y.abs()) as f32).clamp(0.0, 255.0) as u8;

            energy_map.set_value_at(x, y, energy.into());
        }
    }

    energy_map
}
