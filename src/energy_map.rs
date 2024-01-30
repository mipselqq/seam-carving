use image::{Pixel, RgbImage};
use crate::matrix::Matrix;

const LEFT: u32 = 0;
const MID: u32 = 1;
const RIGHT: u32 = 2;

pub fn calculate_energy_map(image: &RgbImage, is_reduced_precision: bool) -> Matrix<u8> {
    let (width, height) = image.dimensions();
    let mut energy_map = Matrix::new(height, width, vec![0; (height * width) as usize]);

    let image_buffer: Vec<_> = image.pixels().collect();

    let sobel_coefficients = [-1, 0, 1];

    let surroundings: Vec<u32> = if is_reduced_precision {
        vec![MID]
    } else {
        vec![LEFT, MID, RIGHT]
    };

    for y in 0..height {
        for x in 0..width {
            let mut gradient_x = 0i16;
            let mut gradient_y = 0i16;

            for x_offset in &surroundings {
                for y_offset in &surroundings {
                    let x_pos = (x + x_offset).max(0).min(width - 1);
                    let y_pos = (y + y_offset).max(0).min(height - 1);
                    let pixel_intensity = image_buffer[(x_pos + y_pos * width) as usize].to_luma()[0] as i16;

                    gradient_x += sobel_coefficients[*x_offset as usize] * pixel_intensity;
                    gradient_y += sobel_coefficients[*y_offset as usize] * pixel_intensity;
                }
            }

            let energy = (gradient_x.abs().max(gradient_y.abs()) as f32).clamp(0.0, 255.0) as u8;

            energy_map.set_value_at(x, y, energy.into());
        }
    }

    energy_map
}
