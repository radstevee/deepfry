use std::{io, path::PathBuf};

use clap::Parser;
use clap_num::number_range;
use deepfry::{deepfry, ChangeMode, DeepfryAlgorithm::BitChange};

fn parse_shift_value(s: &str) -> Result<u8, String> {
    number_range(s, 0, 8)
}

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
    #[arg(short = 'r', value_name = "red", value_parser=parse_shift_value)]
    red: u8,

    /// The green shift.
    #[arg(short = 'g', value_name = "green", value_parser=parse_shift_value)]
    green: u8,

    /// The blue shift.
    #[arg(short = 'b', value_name = "blue", value_parser=parse_shift_value)]
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
        BitChange(args.mode, args.red, args.green, args.blue),
    )?;

    image.save(args.output).unwrap();

    Ok(())
}
