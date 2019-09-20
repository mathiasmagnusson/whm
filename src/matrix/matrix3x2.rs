use crate::Float;
use crate::Vector2;
use crate::Vector3;

use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Matrix3x2([Vector3; 2]);

impl Matrix3x2 {
	pub const fn zero() -> Self {
		Self([Vector3::new(0.0, 0.0, 0.0); 2])
	}
	pub const fn identity() -> Self {
		Self([
			Vector3::new(1.0, 0.0, 0.0),
			Vector3::new(0.0, 1.0, 0.0),
		])
	}
	pub fn solve(mut self) -> Result<Vector2, ()> {
		// Gauss part
		for rn in 0..Self::height() {
			if self[rn][rn] == 0.0 {
				let row = (rn+1..Self::height()).find(|row| self[*row][rn] != 0.0);
				if let Some(row) = row {
					let tmp = self[row];
					self[row] = self[rn];
					self[rn] = tmp;
				} else {
					return Err(());
				}
			}
			self[rn] /= self[rn][rn];
			for row in rn+1..Self::height() {
				self[row] -= self[rn] * self[row][rn];
			}
		}

		// Jordan part
		for rn in (1..Self::height()).rev() {
			for row in (0..rn).rev() {
				self[row] -= self[rn] * self[row][rn];
			}
		}

		Ok(Vector2::new(
			self[0][Self::width() - 1],
			self[1][Self::width() - 1],
		))
	}
	pub fn width() -> usize {
		Vector3::len()
	}
	pub fn height() -> usize {
		2
	}
}

impl From<[Vector3; 2]> for Matrix3x2 {
	fn from(v: [Vector3; 2]) -> Self {
		Self(v)
	}
}

impl From<[[Float; 3]; 2]> for Matrix3x2 {
	fn from(v: [[Float; 3]; 2]) -> Self {
		Self([
			Vector3::from(v[0]),
			Vector3::from(v[1]),
		])
	}
}

impl From<[Float; 3*2]> for Matrix3x2 {
	fn from(v: [Float; 3*2]) -> Self {
		Self([
			Vector3::new(v[0], v[1], v[2]),
			Vector3::new(v[3], v[4], v[5]),
		])
	}
}

impl std::ops::Index<usize> for Matrix3x2 {
	type Output = Vector3;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl std::ops::IndexMut<usize> for Matrix3x2 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl std::ops::Neg for Matrix3x2 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		[
			-self[0],
			-self[1],
		].into()
	}
}

impl fmt::Debug for Matrix3x2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Matrix3x2")?;
		writeln!(f, "┌╴    ╷ ╶┐")?;
		writeln!(f, "│{: >2} {: >2}│{: >2}│", self[0][0], self[0][1], self[0][2])?;
		writeln!(f, "│     │  │")?;
		writeln!(f, "│{: >2} {: >2}│{: >2}│", self[1][0], self[1][1], self[1][2])?;
		writeln!(f, "└╴    ╵ ╶┘")?;

		Ok(())
	}
}
