use std::io;

use clap::ValueEnum;
use image::{Rgb, RgbImage};

/// The direction to shift bits in.
#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum ShiftDirection {
    Left,
    Right,
    Not,
    Multiply,
}

impl ShiftDirection {
    pub fn shift(self, value: u8, other: u8) -> u8 {
        match self {
            Self::Left => value << other,
            Self::Right => value >> other,
            Self::Not => !value,
            Self::Multiply => value * other,
        }
    }
}

#[derive(Debug, Clone)]
/// The algorithm to use while deepfrying.
pub enum DeepfryAlgorithm {
    /// Shifts all the RGB bits by n.
    Bitshift(ShiftDirection, u8, u8, u8),
}

/// Deepfries an image in place.
pub fn deepfry(image: &mut RgbImage, algo: DeepfryAlgorithm) -> io::Result<()> {
    for rgb in image.pixels_mut() {
        let red = rgb.0[0];
        let green = rgb.0[1];
        let blue = rgb.0[2];

        let new_rgb = match algo {
            DeepfryAlgorithm::Bitshift(direction, r, g, b) => {
                let new_red = direction.shift(red, r);
                let new_green = direction.shift(green, g);
                let new_blue = direction.shift(blue, b);
                (new_red, new_green, new_blue)
            }
        };

        *rgb = Rgb([new_rgb.0, new_rgb.1, new_rgb.2])
    }

    Ok(())
}
