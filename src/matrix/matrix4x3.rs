use crate::Float;
use crate::Vector3;

use super::Matrix;

const W: usize = 4;
const H: usize = 3;

pub struct Matrix4x3 {
	v: [Float; W * H],
}

impl Matrix4x3 {
	pub const fn zero() -> Self {
		Self { v: [0.0; W * H] }
	}
	pub const fn identity() -> Self {
		Self {
			v: [
				1., 0., 0., 0., // x
				0., 1., 0., 0., // y
				0., 0., 1., 0., // z
			]
		}
	}
	pub fn solve(mut self) -> Vector3 {
		if self[(0, 0)] != 1.0 {
			self[(3, 0)] /= self[(0, 0)];
			self[(2, 0)] /= self[(0, 0)];
			self[(1, 0)] /= self[(0, 0)];
			self[(0, 0)] /= self[(0, 0)];
		}
		let f = -self[(0, 1)];
		self[(0, 1)] += self[(0, 0)] * f;
		self[(1, 1)] += self[(1, 0)] * f;
		self[(2, 1)] += self[(2, 0)] * f;
		self[(3, 1)] += self[(3, 0)] * f;

		let f = -self[(0, 2)];
		self[(0, 2)] += self[(0, 0)] * f;
		self[(1, 2)] += self[(1, 0)] * f;
		self[(2, 2)] += self[(2, 0)] * f;
		self[(3, 2)] += self[(3, 0)] * f;

		if self[(1, 1)] != 1.0 {
			self[(3, 1)] /= self[(1, 1)];
			self[(2, 1)] /= self[(1, 1)];
			self[(0, 1)] /= self[(1, 1)];
			self[(1, 1)] /= self[(1, 1)];
		}

		let f = -self[(1, 2)];
		self[(0, 2)] += self[(0, 1)] * f;
		self[(1, 2)] += self[(1, 1)] * f;
		self[(2, 2)] += self[(2, 1)] * f;
		self[(3, 2)] += self[(3, 1)] * f;

		if self[(2, 2)] != 1.0 {
			self[(3, 2)] /= self[(2, 2)];
			self[(1, 2)] /= self[(2, 2)];
			self[(0, 2)] /= self[(2, 2)];
			self[(2, 2)] /= self[(2, 2)];
		}

		let f = -self[(2, 0)];
		self[(0, 0)] += self[(0, 2)] * f;
		self[(1, 0)] += self[(1, 2)] * f;
		self[(2, 0)] += self[(2, 2)] * f;
		self[(3, 0)] += self[(3, 2)] * f;

		let f = -self[(2, 1)];
		self[(0, 1)] += self[(0, 2)] * f;
		self[(1, 1)] += self[(1, 2)] * f;
		self[(2, 1)] += self[(2, 2)] * f;
		self[(3, 1)] += self[(3, 2)] * f;

		let f = -self[(1, 0)];
		self[(0, 0)] += self[(0, 1)] * f;
		self[(1, 0)] += self[(1, 1)] * f;
		self[(2, 0)] += self[(2, 1)] * f;
		self[(3, 0)] += self[(3, 1)] * f;

		Vector3::new(self[(3, 0)], self[(3, 1)], self[(3, 2)])
	}
}

impl Matrix for Matrix4x3 {
	fn width(&self) -> usize {
		W
	}
	fn height(&self) -> usize {
		H
	}
	fn get(&self, col: usize, row: usize) -> &Float {
		assert!(
			col < W,
			"Tried to get collumn {} of a Matrix with the width {}",
			col, W
		);
		assert!(
			row < H,
			"Tried to get row {} of a Matrix with the height {}",
			row, H
		);
		&self.v[col + row * H]
	}
	fn get_mut(&mut self, col: usize, row: usize) -> &mut Float {
		assert!(
			col < W,
			"Tried to get collumn {} of a Matrix with the width {}",
			col, W
		);
		assert!(
			row < H,
			"Tried to get row {} of a Matrix with the height {}",
			row, H
		);
		&mut self.v[col + row * H]
	}
}

impl From<[Float; W * H]> for Matrix4x3 {
	fn from(v: [Float; W * H]) -> Self {
		Self { v }
	}
}

impl std::ops::Index<usize> for Matrix4x3 {
	type Output = Float;
	fn index(&self, i: usize) -> &Self::Output {
		&self.v[i]
	}
}

impl std::ops::IndexMut<usize> for Matrix4x3 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.v[i]
	}
}

impl std::ops::Index<(usize, usize)> for Matrix4x3 {
	type Output = Float;
	fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
		self.get(col, row)
	}
}

impl std::ops::IndexMut<(usize, usize)> for Matrix4x3 {
	fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
		self.get_mut(col, row)
	}
}
