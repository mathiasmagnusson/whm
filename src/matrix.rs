mod matrix3x4;
mod matrix3x3;
mod matrix2x2;
mod matrix4x4;

pub use matrix3x4::*;
pub use matrix3x3::*;
pub use matrix2x2::*;
pub use matrix4x4::*;

use crate::Float;

pub trait Matrix {
	fn zero() -> Self;
	fn identity() -> Self;
	fn width() -> usize;
	fn height() -> usize;
	fn det(&self) -> Float;
}
