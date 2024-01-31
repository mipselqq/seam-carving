use carving_image_view::CarvingImageView;
use clap::Parser;
use pbr::ProgressBar;
use std::path::Path;
use seam_removal::remove_seams_up_to;

mod energy_map;
mod seam_removal;
mod matrix;
mod carving_image_view;

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

    let image = image::open(source_image_path).expect("Failed to read the image").into_rgb8();
    let mut carving_image_view = CarvingImageView::from_image(image);

    let mut prograss_bar = ProgressBar::new(target_width.into());

    remove_seams_up_to(&mut carving_image_view, target_width, target_height, false, || {
        prograss_bar.inc();
    });

    carving_image_view.full_image.save(output_path).unwrap();

    prograss_bar.finish_print("done");
}
