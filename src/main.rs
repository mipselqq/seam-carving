mod energy_map;
use std::{path::Path, time::Instant};

use energy_map::calculate_energy_map;

fn main() {
    let source_image_path = Path::new("./images/tower.jpg");
    let source_image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();

    let energy_map_calculation_time = Instant::now();
    let energy_map = calculate_energy_map(&source_image);
    println!("Energy map is calculated in {}ms", energy_map_calculation_time.elapsed().as_millis());

    energy_map.save("energy_map.png").expect("Cannot save the energy map");   
}
