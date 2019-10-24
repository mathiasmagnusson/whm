mod matrix3x4;
mod matrix2x3;
mod matrix2x2;
mod matrix4x4;

pub use matrix3x4::*;
pub use matrix2x3::*;
pub use matrix2x2::*;
pub use matrix4x4::*;

use crate::vector::*;

use std::ops;

pub trait Matrix {
	fn zero() -> Self;
	fn identity() -> Self;
	fn width() -> usize;
	fn height() -> usize;
}

macro_rules! def_matrix {
	($mat: ident, $vec: ident, $n: literal, $m: literal) => {
		pub struct $mat([$vec; $n]);

		impl Matrix for $mat {
			fn height() -> usize {
				$n
			}
			fn width() -> usize {
				$vec::len()
			}
			fn zero() -> Self {
				$mat([$vec::zero(); $n])
			}
			fn identity() -> Self {
				assert_eq!($n, $m);

				let mut this = Self::zero();

				for i in 0..$n {
					this[i][i] = 1.0;
				}

				this
			}
		}

		impl ops::Index<usize> for $mat {
			type Output = $vec;
			fn index(&self, i: usize) -> &Self::Output {
				&self.0[i]
			}
		}

		impl ops::IndexMut<usize> for $mat {
			fn index_mut(&mut self, i: usize) -> &mut Self::Output {
				&mut self.0[i]
			}
		}
	};
}

def_matrix!(Matrix2x2, Vector2, 2, 2);
def_matrix!(Matrix3x3, Vector3, 3, 3);
