use crate::{Float, Vector2};

use super::Matrix;

use std::{fmt, ops};

#[derive(Clone, PartialEq)]
pub struct Matrix2x2([Vector2; 2]);

impl Matrix2x2 {
}

impl Matrix for Matrix2x2 {
	fn zero() -> Self {
		Self([Vector2::new(0.0, 0.0); 2])
	}
	fn identity() -> Self {
		Self([
			Vector2::new(1.0, 0.0),
			Vector2::new(0.0, 1.0),
		])
	}
	fn width() -> usize {
		Vector2::len()
	}
	fn height() -> usize {
		2
	}
	fn det(&self) -> Float {
		self[0][0] * self[1][1] - self[0][1] * self[1][0]
	}
}

impl From<[Vector2; 2]> for Matrix2x2 {
	fn from(v: [Vector2; 2]) -> Self {
		Self(v)
	}
}

impl From<[[Float; 2]; 2]> for Matrix2x2 {
	fn from(v: [[Float; 2]; 2]) -> Self {
		Self([
			Vector2::from(v[0]),
			Vector2::from(v[1]),
		])
	}
}

impl From<[Float; 2*2]> for Matrix2x2 {
	fn from(v: [Float; 2*2]) -> Self {
		Self([
			Vector2::new(v[0], v[1]),
			Vector2::new(v[2], v[3]),
		])
	}
}

impl ops::Index<usize> for Matrix2x2 {
	type Output = Vector2;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl ops::IndexMut<usize> for Matrix2x2 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl fmt::Debug for Matrix2x2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "┌╴        ╶┐")?;
		writeln!(f, "│{: >2} {: >2} {: >2} │", self[0][0], self[0][1], self[0][2])?;
		writeln!(f, "│          │")?;
		writeln!(f, "│{: >2} {: >2} {: >2} │", self[1][0], self[1][1], self[1][2])?;
		writeln!(f, "└╴        ╶┘")?;

		Ok(())
	}
}

impl ops::Neg for Matrix2x2 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		[
			-self[0],
			-self[1],
		].into()
	}
}

impl ops::Add for Matrix2x2 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		[
			self[0] + rhs[0],
			self[1] + rhs[1],
		].into()
	}
}

impl ops::AddAssign for Matrix2x2 {
	fn add_assign(&mut self, rhs: Self) {
		self[0] += rhs[0];
		self[1] += rhs[1];
	}
}

impl ops::Sub for Matrix2x2 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		[
			self[0] - rhs[0],
			self[1] - rhs[1],
		].into()
	}
}

impl ops::SubAssign for Matrix2x2 {
	fn sub_assign(&mut self, rhs: Self) {
		self[0] -= rhs[0];
		self[1] -= rhs[1];
	}
}

impl ops::Mul for Matrix2x2 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		fn calc_element(a: &Matrix2x2, b: &Matrix2x2, r: usize, c: usize) -> Float {
			(0..Matrix2x2::width()).map(|i| a[r][i]*b[i][c]).sum()
		}
		[
			calc_element(&self, &rhs, 0, 0), calc_element(&self, &rhs, 0, 1),
			calc_element(&self, &rhs, 1, 0), calc_element(&self, &rhs, 1, 1),
		].into()
	}
}
