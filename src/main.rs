mod energy_map;
mod seam_removal;
use std::{path::Path, time::Instant};

use energy_map::calculate_energy_map;

use crate::seam_removal::remove_vertical_seam;

const TARGET_WIDTH: u32 = 1000;

fn main() {
    let source_image_path = Path::new("./images/tower.jpg");
    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();

    let energy_map_calculation_time = Instant::now();
    let mut energy_map = calculate_energy_map(&image);
    println!("Energy map is calculated in {}ms", energy_map_calculation_time.elapsed().as_millis());

    for i in 0..(image.width() - TARGET_WIDTH)  {
        remove_vertical_seam(&mut image, &energy_map);
        energy_map = calculate_energy_map(&image);
        dbg!("removed", i);
    }

    image.save("vertical_carving.png").unwrap();
}
