use std::time::Instant;

use image::RgbImage;

use crate::{energy_map::calculate_energy_map, matrix::Matrix};

#[derive(Debug)]
pub struct Seam {
    pub x: u32,
    pub y: u32,
}

pub fn remove_seams_up_to<F: Fn(u32)>(image: &mut RgbImage, width: u32, height: u32, callback: F) {
    while image.width() > width {
        callback(image.width());

        let energy_map = calculate_energy_map(image, false);
        let seam = find_vertical_seam(energy_map);

        remove_vertical_seam(image, seam);
    }
}

pub fn remove_vertical_seam(image: &mut RgbImage, seam: Vec<Seam>) {
    let (width, height) = image.dimensions();

    for Seam { x, y } in seam {
        for i in x..width - 1 {
            let p = image.get_pixel(i + 1, y);
            image.put_pixel(i, y, *p);
        }
    }

    let sub_image = image::imageops::crop(image, 0, 0, width - 1, height);
    *image = sub_image.to_image();
}

pub fn find_vertical_seam(energy_map: Matrix<u8>) -> Vec<Seam> {
    let (dp_table, min_indices) = make_dp_table(energy_map);
    let seams = traverse_back_dp_table(&dp_table, &min_indices);

    seams
}

fn make_dp_table(energy_map: Matrix<u8>) -> (Matrix<u32>, Matrix<i32>) {
    let height = energy_map.height;
    let width = energy_map.width;

    let mut dp_table = Matrix::new(height, width, vec![0u32; (width * height) as usize]);
    let mut path_table = Matrix::new(height, width, vec![0i32; (width * height) as usize]);

    for y in 1..energy_map.height {
        for x in 0..width {
            let current_energy = energy_map.get_value_at(x, y);

            let energy_above_left = if x > 0 {
                *dp_table.get_value_at(x - 1, y - 1)
            } else {
                std::u32::MAX 
            };

            let energy_above_mid = *dp_table.get_value_at(x, y - 1);

            let energy_above_right = if x < width - 1 {
                *dp_table.get_value_at(x + 1, y - 1)
            } else {
                std::u32::MAX
            };

            let energies = [energy_above_left, energy_above_mid, energy_above_right];

            let min_index = energies.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap().0;
            let min_energy = energies[min_index];
            let offset = min_index as i32 - 1;
            let total_energy = *current_energy as u32 + min_energy;

            dp_table.set_value_at(x, y, total_energy);
            path_table.set_value_at(x, y, offset);
        }
    }

    (dp_table, path_table)
}


fn traverse_back_dp_table(dp_table: &Matrix<u32>, path_table: &Matrix<i32>) -> Vec<Seam> {
    let mut seams = Vec::new();
    let height = dp_table.height;

    let mut current_x = dp_table.min_index_in_row(height - 1);
    for y in (0..height).rev() {
        seams.push(Seam { x: current_x, y });

        current_x = (current_x as i32 + *path_table.get_value_at(current_x, y)) as u32;
    }

    seams.reverse();

    seams
}
