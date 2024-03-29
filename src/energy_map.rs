use crate::{matrix::Matrix, seam_removal::SeamPixel};
use image::{GenericImageView, Pixel, RgbImage, SubImage};

const LEFT: u32 = 0;
const MID: u32 = 1;
const RIGHT: u32 = 2;
const SURROUNDINGS: [u32; 3] = [LEFT, MID, RIGHT];

pub fn generate_energy_map(sub_image: &mut SubImage<&mut RgbImage>) -> Matrix<u8> {
    let (width, height) = sub_image.dimensions();
    let mut energy_map = Matrix::new(height, width, vec![0; (height * width) as usize]);

    let sobel_coefficients = [-1, 0, 1];

    for y in 0..height {
        for x in 0..width {
            let mut gradient_x = 0i16;
            let mut gradient_y = 0i16;

            for x_offset in SURROUNDINGS {
                for y_offset in SURROUNDINGS {
                    let x_pos = (x + x_offset).max(0).min(width - 1);
                    let y_pos = (y + y_offset).max(0).min(height - 1);
                    // It's totally safe (I guess)
                    let pixel_intensity =
                        unsafe { sub_image.unsafe_get_pixel(x_pos, y_pos).to_luma() }[0] as i16;

                    gradient_x += sobel_coefficients[x_offset as usize] * pixel_intensity;
                    gradient_y += sobel_coefficients[y_offset as usize] * pixel_intensity;
                }
            }

            let energy = (gradient_x.abs().max(gradient_y.abs()) as f32).clamp(0.0, 255.0) as u8;

            energy_map.set_value_at(x, y, energy);
        }
    }

    energy_map
}

pub fn remove_vertical_seam<'a>(energy_map: &'a mut Matrix<u8>, seam: &'a Vec<SeamPixel>) {
    let (width, height) = energy_map.dimensions();

    for SeamPixel { x, y } in seam {
        for i in *x..width - 1 {
            let p = energy_map.get_value_at(i + 1, *y);
            energy_map.set_value_at(i, *y, *p);
        }
    }

    energy_map.crop(0, 0, width - 1, height);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImage, Rgb, RgbImage};

    #[test]
    fn energy_map_generation() {
        let mut image = RgbImage::new(5, 5);

        let mask = [
            [1, 0, 1, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let color = 255 * mask[y as usize][x as usize];

            *pixel = Rgb([color, color, color]);
        }

        let mut sub_image = image.sub_image(0, 0, 5, 5);

        let energy_map = generate_energy_map(&mut sub_image);

        #[rustfmt::skip]
        let expected = Matrix{ 
            height: 5,
            width: 5,
            data: vec![
                255, 255, 255, 255, 255,
                255, 255, 255, 0,   0,
                255, 255, 255, 255, 255,
                255, 255, 0,   0,   0,
                0,   0,   0,   0,   0,
            ]
        };

        assert_eq!(energy_map, expected);
    }
}
