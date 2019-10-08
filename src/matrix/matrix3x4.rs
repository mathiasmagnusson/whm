use crate::Float;
use crate::Vector3;
use crate::Vector4;

use std::{fmt, ops};

#[derive(Clone, PartialEq)]
pub struct Matrix3x4([Vector4; 3]);

impl Matrix3x4 {
	pub const fn zero() -> Self {
		Self([Vector4::new(0.0, 0.0, 0.0, 0.0); 3])
	}
	pub const fn identity() -> Self {
		Self([
			Vector4::new(1.0, 0.0, 0.0, 0.0),
			Vector4::new(0.0, 1.0, 0.0, 0.0),
			Vector4::new(0.0, 0.0, 1.0, 0.0),
		])
	}
	pub fn solve(mut self) -> Result<Vector3, ()> {
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
			let v = self[rn][rn];
			self[rn] /= v;
			for row in rn+1..Self::height() {
				let v = self[rn] * self[row][rn];
				self[row] -= v;
			}
		}

		// Jordan part
		for rn in (1..Self::height()).rev() {
			for row in (0..rn).rev() {
				let v = self[rn] * self[row][rn];
				self[row] -= v;
			}
		}

		Ok(Vector3::new(self[0][3], self[1][3], self[2][3]))
	}
	pub fn width() -> usize {
		Vector4::len()
	}
	pub fn height() -> usize {
		3
	}
}

impl From<[Vector4; 3]> for Matrix3x4 {
	fn from(v: [Vector4; 3]) -> Self {
		Self(v)
	}
}

impl From<[[Float; 4]; 3]> for Matrix3x4 {
	fn from(v: [[Float; 4]; 3]) -> Self {
		Self([
			Vector4::from(v[0]),
			Vector4::from(v[1]),
			Vector4::from(v[2]),
		])
	}
}

impl From<[Float; 4*3]> for Matrix3x4 {
	fn from(v: [Float; 4*3]) -> Self {
		Self([
			Vector4::new(v[0], v[1], v[2], v[3]),
			Vector4::new(v[4], v[5], v[6], v[7]),
			Vector4::new(v[8], v[9], v[10], v[11]),
		])
	}
}

impl ops::Index<usize> for Matrix3x4 {
	type Output = Vector4;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl ops::IndexMut<usize> for Matrix3x4 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl ops::Neg for Matrix3x4 {
	type Output = Matrix3x4;
	fn neg(self) -> Self::Output {
		[
			-self[0],
			-self[1],
			-self[2],
		].into()
	}
}

impl ops::Add for Matrix3x4 {
	type Output = Matrix3x4;
	fn add(self, rhs: Matrix3x4) -> Self::Output {
		[
			self[0] + rhs[0],
			self[1] + rhs[1],
			self[2] + rhs[2],
		].into()
	}
}

impl ops::AddAssign for Matrix3x4 {
	fn add_assign(&mut self, rhs: Matrix3x4) {
		self[0] += rhs[0];
		self[1] += rhs[1];
		self[2] += rhs[2];
	}
}

impl fmt::Debug for Matrix3x4 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// write!(
		// 	f,
		// 	"Matrix3x4 [\n    {}, {}, {}, {},\n    {}, {}, {}, {},\n    {}, {}, {}, {},\n]",
		// 	self[0][0], self[0][1], self[0][2], self[0][3],
		// 	self[1][0], self[1][1], self[1][2], self[1][3],
		// 	self[2][0], self[2][1], self[2][2], self[2][3],
		// )
		writeln!(f, "Matrix3x4")?;
		writeln!(f, "┌╴       ╷ ╶┐")?;
		writeln!(f, "│{: >2} {: >2} {: >2}│{: >2}│", self[0][0], self[0][1], self[0][2], self[0][3])?;
		writeln!(f, "│        │  │")?;
		writeln!(f, "│{: >2} {: >2} {: >2}│{: >2}│", self[1][0], self[1][1], self[1][2], self[1][3])?;
		writeln!(f, "│        │  │")?;
		writeln!(f, "│{: >2} {: >2} {: >2}│{: >2}│", self[2][0], self[2][1], self[2][2], self[2][3])?;
		writeln!(f, "└╴       ╵ ╶┘")?;

		Ok(())
	}
}
