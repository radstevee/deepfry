use std::io;

use clap::ValueEnum;
use image::{Rgb, RgbImage};

/// The mode for Bit changing.
#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum ChangeMode {
    /// Shifts bits to the left.
    ShiftLeft,
    /// Shifts bits to the right.
    ShiftRight,
    /// Does a NOT operation on the bits.
    Not,
    /// Multiplies the bits.
    Multiply,
    /// Uses the square root of the bits.
    Sqrt,
    /// Does an XOR operation on the bits.
    Xor,
    /// Does an OR operation on the bits.
    Or,
    /// Does an AND operation on the bits.
    And,
    /// Raises the bits to the power of the other provided value
    Exponent,
}

impl ChangeMode {
    pub fn shift(self, value: u8, other: u8) -> u8 {
        match self {
            Self::ShiftLeft => value.wrapping_shl(other.into()),
            Self::ShiftRight => value.wrapping_shr(other.into()),
            Self::Not => !value,
            Self::Multiply => value.wrapping_mul(other),
            Self::Sqrt => sqrt(value),
            Self::Xor => value ^ other,
            Self::Or => value | other,
            Self::And => value & other,
            Self::Exponent => value.wrapping_pow(other.into()),
        }
    }
}

/// The algorithm to use while deepfrying.
#[derive(Debug, Clone)]
pub enum DeepfryAlgorithm {
    /// Changes bits based off a ChangeMode.
    BitChange(ChangeMode, u8, u8, u8),
}

/// Deepfries an image in place.
pub fn deepfry(image: &mut RgbImage, algo: DeepfryAlgorithm) -> io::Result<()> {
    for rgb in image.pixels_mut() {
        let red = rgb.0[0];
        let green = rgb.0[1];
        let blue = rgb.0[2];

        let new_rgb = match algo {
            DeepfryAlgorithm::BitChange(direction, r, g, b) => {
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

fn sqrt(value: u8) -> u8 {
    (value as f32).sqrt() as u8
}