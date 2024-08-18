use std::{io, path::PathBuf};

use clap::Parser;
use deepfry::{deepfry, ChangeMode};

/// Deepfry - A tool for deepfrying images.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file.
    #[arg(value_name = "input")]
    input: PathBuf,

    /// The output file.
    #[arg(value_name = "output")]
    output: PathBuf,

    /// The red shift.
    #[arg(short = 'r', value_name = "red")]
    red: u8,

    /// The green shift.
    #[arg(short = 'g', value_name = "green")]
    green: u8,

    /// The blue shift.
    #[arg(short = 'b', value_name = "blue")]
    blue: u8,

    /// The bit changing mode.
    #[arg(short = 'm', value_name = "mode")]
    mode: ChangeMode,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut image = image::open(args.input).unwrap().to_rgb8();

    deepfry(
        &mut image,
        deepfry::DeepfryAlgorithm::BitChange(args.mode, args.red, args.green, args.blue),
    )?;

    image.save(args.output).unwrap();

    Ok(())
}
