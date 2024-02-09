use clap::Parser;
use image::GenericImage;
use pbr::ProgressBar;
use std::path::Path;
use seam_removal::remove_seams_up_to;

mod energy_map;
mod seam_removal;
mod matrix;
mod types;

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

    #[arg(short, long)]
    fast: u8,
}

fn main() {
    let args = Args::parse();

    let source_image_path = Path::new(&args.input);
    let target_width = args.width;
    let target_height = args.height;
    let output_path = args.output;
    let is_fast = args.fast == 1;

    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();
    let (width, height) = image.dimensions();

    let mut prograss_bar = ProgressBar::new((width - target_width).into());

    let carved = remove_seams_up_to(&mut image, target_width, target_height, !is_fast, || {
        prograss_bar.inc();
    });

    carved.save(output_path).unwrap();

    prograss_bar.finish_print("done");
}
