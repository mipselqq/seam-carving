use image::{Pixel, RgbImage};
use crate::matrix::Matrix;

pub fn calculate_energy_map(image: &RgbImage) -> Matrix<u8> {
    let (width, height) = image.dimensions();
    let mut energy_map = Matrix::new(height, width, vec![0; (height * width) as usize]);

    for y in 0..height {
        for x in 0..width {
            let mut gradient_x = 0i16;
            let mut gradient_y = 0i16;

            for x_offset in 0..3 {
                for y_offset in 0..3 {
                    let px = (x as i32 + x_offset as i32 - 1).max(0).min((width - 1) as i32) as u32;
                    let py = (y as i32 + y_offset as i32 - 1).max(0).min((height - 1) as i32) as u32;

                    let pixel_intensity = image.get_pixel(px, py).to_luma()[0] as i16;

                    gradient_x += calculate_sobel_coefficient(x_offset) * pixel_intensity;
                    gradient_y += calculate_sobel_coefficient(y_offset) * pixel_intensity;
                }
            }

            let energy = (gradient_x.abs().max(gradient_y.abs()) as f32).clamp(0.0, 255.0) as u8;

            energy_map.set_value_at(x, y, energy.into());
        }
    }

    energy_map
}

fn calculate_sobel_coefficient(axis_offset: u32) -> i16 {
    match axis_offset {
        0 => -1,
        1 => 0,
        _ => 1,
    }
}
