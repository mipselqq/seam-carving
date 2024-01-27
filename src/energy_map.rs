use image::{GrayImage, Luma, Pixel, RgbImage};

pub fn calculate_energy_map(image: &RgbImage) -> GrayImage {
    let (image_width, image_height) = image.dimensions();
    let mut energy_map = GrayImage::new(image_width, image_height);

    for y in 1..(image_height - 1) {
        for x in 1..(image_width - 1) {
            let mut gradient_x = 0i16;
            let mut gradient_y = 0i16;

            for x_offset in 0..3 {
                for y_offset in 0..3 {
                    let pixel_intensity = image.get_pixel(x + x_offset as u32 - 1, y + y_offset as u32 - 1).to_luma()[0] as i16;

                    gradient_x += calculate_sobel_coefficient(x_offset) * pixel_intensity;
                    gradient_y += calculate_sobel_coefficient(y_offset) * pixel_intensity;
                }
            }

            let energy = (gradient_x.abs().max(gradient_y.abs()) as f32).clamp(0.0, 255.0) as u8;

            energy_map.put_pixel(x, y, Luma([energy]));
        }
    }

    energy_map
}

fn calculate_sobel_coefficient(axis_offset: i16) -> i16 {
    match axis_offset {
        0 => -1,
        1 => 0,
        _ => 1,
    }
}
