use image::{
    imageops::{rotate270, rotate90},
    GenericImage, GenericImageView, RgbImage, SubImage,
};

use crate::{energy_map::generate_energy_map, matrix::Matrix};

#[derive(Debug, Clone)]
pub struct SeamPixel {
    pub x: u32,
    pub y: u32,
}

pub fn remove_seams_up_to_targets(
    image: &mut RgbImage,
    target_width: u32,
    target_height: u32,
    recalculate_energy: bool,
    on_width_change: impl FnMut(),
    on_height_change: impl FnMut(),
) -> RgbImage {
    let width_carved_image =
        remove_seams_up_to_target_width(image, target_width, recalculate_energy, on_width_change);
    // Swap an image's height with its width
    let mut rotated_image = rotate90(&width_carved_image);
    // Repeat the same thing with the rotated image
    let height_carved_image = remove_seams_up_to_target_width(
        &mut rotated_image,
        target_height,
        recalculate_energy,
        on_height_change,
    );
    // Rotate it back
    rotate270(&height_carved_image)
}

fn remove_seams_up_to_target_width<F: FnMut()>(
    image: &mut RgbImage,
    target_width: u32,
    recalculate_energy: bool,
    mut callback: F,
) -> RgbImage {
    let (width, height) = image.dimensions();

    let sub_image = &mut image.sub_image(0, 0, width, height);
    let mut energy_map = generate_energy_map(sub_image);

    while sub_image.width() > target_width {
        let seam = find_vertical_seam_with_lowest_energy(&energy_map);

        if recalculate_energy {
            energy_map = generate_energy_map(sub_image);
        } else {
            crate::energy_map::remove_vertical_seam(&mut energy_map, &seam);
        }

        self::remove_vertical_seam(sub_image, seam);

        callback();
    }

    sub_image.to_image()
}

fn remove_vertical_seam(sub_image: &mut SubImage<&mut RgbImage>, seam: Vec<SeamPixel>) {
    let (width, height) = sub_image.dimensions();

    for SeamPixel { x, y } in seam {
        for i in x..width - 1 {
            let p = unsafe { sub_image.unsafe_get_pixel(i + 1, y) };
            unsafe { sub_image.unsafe_put_pixel(i, y, p) };
        }
    }

    sub_image.change_bounds(0, 0, width - 1, height);
}

fn find_vertical_seam_with_lowest_energy(energy_map: &Matrix<u8>) -> Vec<SeamPixel> {
    let (dp_table, min_indices) = make_dp_table(energy_map);

    traverse_back_dp_table(&dp_table, &min_indices)
}

fn make_dp_table(energy_map: &Matrix<u8>) -> (Matrix<u32>, Matrix<u32>) {
    let (width, height) = energy_map.dimensions();
    let mut dp_table = Matrix::<u32>::from_dimensions_filled_with(height, width, 0);
    let mut path_table = Matrix::<u32>::from_dimensions_filled_with(height, width, 0);

    for y in 1..height {
        for x in 0..width {
            let current_energy = energy_map.get_value_at(x, y);
            let energies_above = get_energies_above(&dp_table, x, y);
            let (min_energy, offset) = find_min_energy_and_offset(&energies_above);

            let total_energy = *current_energy as u32 + min_energy;

            dp_table.set_value_at(x, y, total_energy);
            path_table.set_value_at(x, y, offset);
        }
    }

    (dp_table, path_table)
}

fn get_energies_above(dp_table: &Matrix<u32>, x: u32, y: u32) -> [u32; 3] {
    let energy_above_left = if x > 0 {
        *dp_table.get_value_at(x - 1, y - 1)
    } else {
        std::u32::MAX
    };

    let energy_above_mid = *dp_table.get_value_at(x, y - 1);

    let energy_above_right = if x < dp_table.width - 1 {
        *dp_table.get_value_at(x + 1, y - 1)
    } else {
        std::u32::MAX
    };

    [energy_above_left, energy_above_mid, energy_above_right]
}

fn find_min_energy_and_offset(energies: &[u32; 3]) -> (u32, u32) {
    let min_index = energies
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .unwrap()
        .0;
    let min_energy = energies[min_index];
    let offset = min_index as u32 - 1;

    (min_energy, offset)
}

fn traverse_back_dp_table(dp_table: &Matrix<u32>, path_table: &Matrix<u32>) -> Vec<SeamPixel> {
    let height = dp_table.height;
    let mut seams = Vec::with_capacity(height as usize);

    let mut current_x = dp_table.min_index_in_row(height - 1);
    for y in (0..height).rev() {
        seams.push(SeamPixel { x: current_x, y });

        current_x += path_table.get_value_at(current_x, y);
    }

    seams.reverse();

    seams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_vertical_seam() {
        let mut image = RgbImage::new(10, 10);
        let mut sub_image = SubImage::new(&mut image, 0, 0, 10, 10);
        let seam: Vec<SeamPixel> = (0..10).map(|i| SeamPixel { x: 5, y: i }).collect();

        remove_vertical_seam(&mut sub_image, seam);

        assert_eq!(sub_image.width(), 9);
    }

    // TODO: increase test coverage of this module
}
