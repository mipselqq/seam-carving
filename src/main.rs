mod energy_map;
mod seam_removal;
mod matrix;

use std::path::Path;

use seam_removal::remove_seams_up_to;

const TARGET_WIDTH: u32 = 500;
const TARGET_HEIGHT: u32 = 800;

fn main() {
    let source_image_path = Path::new("./images/tower.jpg");
    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();

    remove_seams_up_to(&mut image, TARGET_WIDTH, TARGET_HEIGHT);

    image.save("carved.png").unwrap();
}
