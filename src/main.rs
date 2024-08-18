use std::{io, path::PathBuf};

use clap::Parser;
use deepfry::{deepfry, ShiftDirection};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "input")]
    input: PathBuf,
    #[arg(value_name = "output")]
    output: PathBuf,

    #[arg(short = 'r', value_name = "red")]
    red: u8,
    #[arg(short = 'g', value_name = "green")]
    green: u8,
    #[arg(short = 'b', value_name = "blue")]
    blue: u8,

    #[arg(short = 'd', value_name = "direction")]
    direction: ShiftDirection,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut image = image::open(args.input).unwrap().to_rgb8();

    deepfry(
        &mut image,
        deepfry::DeepfryAlgorithm::Bitshift(args.direction, args.red, args.green, args.blue),
    )?;

    image.save(args.output).unwrap();

    Ok(())
}
