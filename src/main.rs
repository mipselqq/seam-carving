use clap::Parser;
use pbr::ProgressBar;
use seam_removal::remove_seams_up_to_targets;
use userinput_parsing::parse_target_dimension;
use std::path::Path;

mod energy_map;
mod seam_removal;
mod matrix;
mod types;
mod userinput_parsing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(disable_help_flag = true)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    width: String,

    #[arg(short, long)]
    height: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    fast: u8,
}

fn main() {
    let args = Args::parse();

    let source_image_path = Path::new(&args.input);
    let output_path = args.output;
    let is_fast = args.fast == 1;

    let mut image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();
    let (width, height) = image.dimensions();

    let target_width = parse_target_dimension(args.width.to_string(), width);
    let target_height = parse_target_dimension(args.height.to_string(), height);

    let mut width_prograss_bar = ProgressBar::new((width - target_width).into());
    let mut height_prograss_bar = ProgressBar::new((height - target_height).into());

    width_prograss_bar.message("Carving width: ");
    height_prograss_bar.message("Carving height: ");

    let carved = remove_seams_up_to_targets(&mut image, target_width, target_height, !is_fast, || {
        width_prograss_bar.inc();
    }, || {
        height_prograss_bar.inc();
    });

    carved.save(output_path).unwrap();

    height_prograss_bar.finish_print("Done");
}
