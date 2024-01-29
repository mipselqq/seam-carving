use image::RgbImage;

use crate::{energy_map::calculate_energy_map, matrix::Matrix};

#[derive(Debug)]
pub struct Seam {
    pub x: u32,
    pub y: u32,
}

pub fn remove_vertical_seam(image: &mut RgbImage) {
    let energy_map = calculate_energy_map(&image);
    let seam = find_vertical_seam(&energy_map);

    let (width, height) = image.dimensions();

    for Seam { x, y } in seam {
        for i in x..width - 1 {
            let p = image.get_pixel(i + 1, y);
            image.put_pixel(i, y, *p);
        }
    }

    let sub_image = image::imageops::crop_imm(image, 0, 0, width - 1, height);
    *image = sub_image.to_image();
}

pub fn find_vertical_seam(energy_map: &Matrix<u8>) -> Vec<Seam> {
    let (dp_table, min_indices) = make_dp_table(energy_map);
    let seams = traverse_back_dp_table(&dp_table, &min_indices);

    seams
}

fn make_dp_table(energy_map: &Matrix<u8>) -> (Matrix<u32>, Vec<u32>) {
    let height = energy_map.height;
    let width = energy_map.width;

    let mut dp_table = Matrix::new(height, width, vec![0u32; (width * height) as usize]);
    let mut min_indices = vec![0u32; height as usize];

    for y in 1..energy_map.height {
        let mut min_energy = std::u32::MAX;
        let mut min_x = 0;

        for x in 0..width {
            let current_energy = energy_map.value_at(x, y);

            let min_energy_prev;

            if x == 0 {
                min_energy_prev = *dp_table.value_at(x, y - 1)
                        .min(dp_table.value_at(x + 1, y - 1));
            } else if x == width - 1 {
                min_energy_prev = *dp_table.value_at(x - 1, y - 1)
                        .min(dp_table.value_at(x, y - 1));
            } else {
                min_energy_prev = *dp_table.value_at(x - 1, y - 1)
                        .min(dp_table.value_at(x, y - 1))
                        .min(dp_table.value_at(x + 1, y - 1));
            }

            let total_energy = *current_energy as u32 + min_energy_prev;
            dp_table.set_value_at(x, y, total_energy);

            if total_energy < min_energy {
                min_energy = total_energy;
                min_x = x;
            }
        }

        min_indices[y as usize] = min_x;
    }

    (dp_table, min_indices)
}

fn traverse_back_dp_table(dp_table: &Matrix<u32>, min_indices: &[u32]) -> Vec<Seam> {
    let mut seams = Vec::new();
    let height = dp_table.height;

    let mut current_x = min_indices[(height - 1) as usize];
    for y in (0..height).rev() {
        seams.push(Seam { x: current_x, y });

        if y != 0 {
            current_x = min_indices[(y - 1) as usize];
        }
    }

    seams.reverse();
    seams
}
