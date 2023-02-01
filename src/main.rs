extern crate image;
use image::imageops::Nearest;
extern crate clap;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Args {
    #[clap(short, long, default_value = "polars.png")]
    file_name: String,
    #[clap(short, long, default_value = "256")]
    image_size: u32,
}

// create a function that loads image file using image crate and resizes it to desired size

fn main() {
    let args = Args::parse();
    println!("Loading image file: {}", args.file_name);
    let img = image::open(args.file_name).unwrap();
    let img = img.resize(args.image_size, args.image_size, Nearest);
    img.save("polars_resized.png").expect("Failed to save image");
}


