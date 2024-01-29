mod energy_map;
mod seam_removal;
mod matrix;

use std::{path::Path, time::Instant};

use image::imageops;

use crate::seam_removal::remove_vertical_seam;

const TARGET_WIDTH: u32 = 800;
// const TARGET_HEIGHT: u32 = 800;

fn main() {
    let source_image_path = Path::new("./images/tower.jpg");
    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();

    while image.width() > TARGET_WIDTH {
        let now = Instant::now();
        remove_vertical_seam(&mut image);
        println!("V: {} ms", now.elapsed().as_millis())
    }

    // if image.height() > TARGET_HEIGHT {
    //     imageops::rotate90(&image);

    //     while image.height() > TARGET_WIDTH {
    //         let now = Instant::now();
    //         remove_vertical_seam(&mut image);
    //         println!("H: {} ms", now.elapsed().as_millis())
    //     }

    //     imageops::rotate270(&image);
    // }

    image.save("carved.png").unwrap();
}
