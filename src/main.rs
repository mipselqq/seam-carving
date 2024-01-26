mod energy_map;
use energy_map::calculate_energy_map;

fn main() {
    let source_image_path = "arch.jpg";
    let source_image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();
    let energy_map = calculate_energy_map(&source_image);

    energy_map.save("energy_map.png").expect("Cannot save the energy map");   
}
