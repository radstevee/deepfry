use std::io;

use clap::ValueEnum;
use enum_stringify::EnumStringify;
use image::{Rgb, RgbImage};
use rand::Rng;
use rand::SeedableRng;
use serde::Deserialize;
use serde::Serialize;

/// The mode for Bit changing.
#[derive(Debug, Clone, ValueEnum, Copy, EnumStringify)]
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
    /// Adds a random value to the bits, using the other value as a seed.
    RandomAdd,
    /// Multiplies the bits by a random value, using the other value as a seed.
    RandomMul,
}

impl ChangeMode {
    pub fn shift(self, value: u8, other: u32) -> u8 {
        match self {
            Self::ShiftLeft => value.wrapping_shl(other.into()),
            Self::ShiftRight => value.wrapping_shr(other.into()),
            Self::Not => !value,
            Self::Multiply => value.wrapping_mul(other.try_into().unwrap()),
            Self::Sqrt => value.sqrt(),
            Self::Xor => value ^ other as u8,
            Self::Or => value | other as u8,
            Self::And => value & other as u8,
            Self::Exponent => value.wrapping_pow(other.into()),
            Self::RandomAdd => {
                let mut rng = rand::rngs::SmallRng::seed_from_u64(other as u64);
                value.wrapping_add(rng.gen())
            }
            Self::RandomMul => {
                let mut rng = rand::rngs::SmallRng::seed_from_u64(other as u64);
                value.wrapping_mul(rng.gen())
            }
        }
    }
}

/// The algorithm to use while deepfrying.
#[derive(Debug, Clone)]
pub enum DeepfryAlgorithm {
    /// Changes bits based off a ChangeMode.
    BitChange(ChangeMode, u32, u32, u32),
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

trait NumberExt {
    fn sqrt(self) -> Self;
}

impl NumberExt for u8 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt() as u8
    }
}

/// A configuration for an algorithm.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlgorithmConfig {
    pub algorithm: String,
    pub change_mode: Option<String>,
    pub red: Option<u32>,
    pub green: Option<u32>,
    pub blue: Option<u32>,
}

impl AlgorithmConfig {
    pub fn algo(self) -> Result<DeepfryAlgorithm, String> {
        return match self.algorithm.as_str() {
            "BitChange" => {
                if self.change_mode.is_none() {
                    return Err("bit changing mode is not set".to_string());
                }

                let r = self.red.unwrap_or(0);
                let g = self.green.unwrap_or(0);
                let b = self.blue.unwrap_or(0);

                let change_mode = ChangeMode::try_from(self.change_mode.clone().unwrap());

                match change_mode {
                    Ok(change_mode) => Ok(DeepfryAlgorithm::BitChange(change_mode, r, g, b)),
                    Err(_) => Err(format!("invalid bit changing mode: {:?}", self.change_mode)),
                }
            }
            _ => return Err(format!("invalid algorithm: {}", self.algorithm)),
        };
    }
}

/// A preset for deepfrying images using several algorithms
/// and configs without running multiple commands.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preset {
    pub algorithms: Vec<AlgorithmConfig>,
}

