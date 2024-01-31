use clap::Parser;
use std::path::Path;
use seam_removal::remove_seams_up_to;

mod energy_map;
mod seam_removal;
mod matrix;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(disable_help_flag = true)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    width: u32,

    #[arg(short, long)]
    height: u32,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let source_image_path = Path::new(&args.input);
    let target_width: u32 = args.width;
    let target_height: u32 = args.height;
    let output_path = &args.output;

    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();

    remove_seams_up_to(&mut image, target_width, target_height, false, |image_width| {
        let width_left = image_width - target_width;

        println!("Pixels left to carve: {width_left}")
    });

    image.save(output_path).unwrap();
}
