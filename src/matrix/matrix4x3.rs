use crate::Float;
use crate::Vector3;
use crate::Vector4;

use std::fmt;

#[derive(Clone)]
pub struct Matrix4x3 {
	v: [Vector4; 3],
}

impl Matrix4x3 {
	pub const fn zero() -> Self {
		Self { v: [Vector4::new(0.0, 0.0, 0.0, 0.0); 3] }
	}
	pub const fn identity() -> Self {
		Self {
			v: [
				Vector4::new(1.0, 0.0, 0.0, 0.0),
				Vector4::new(0.0, 1.0, 0.0, 0.0),
				Vector4::new(0.0, 0.0, 1.0, 0.0),
			]
		}
	}
	pub fn solve(mut self) -> Result<Vector3, ()> {
		// Gauss part
		for rn in 0..Self::height() {
			// If this row has a 0 in the main
			// diagonal, look through the rest
			// of the rows to find one that does
			// not have a zero in the collumn we
			// are looking for. If one is found,
			// we swap places of these two, otherwise
			// return an error.
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

		Ok(Vector3::new(self[0][3], self[1][3], self[2][3]))
	}
	pub fn width() -> usize {
		Vector4::len()
	}
	pub fn height() -> usize {
		3
	}
}

impl From<[Vector4; 3]> for Matrix4x3 {
	fn from(v: [Vector4; 3]) -> Self {
		Self { v }
	}
}

impl From<[[Float; 4]; 3]> for Matrix4x3 {
	fn from(v: [[Float; 4]; 3]) -> Self {
		Self {
			v: [
				Vector4::from(v[0]),
				Vector4::from(v[1]),
				Vector4::from(v[2]),
			]
		}
	}
}

impl From<[Float; 4*3]> for Matrix4x3 {
	fn from(v: [Float; 4*3]) -> Self {
		Self {
			v: [
				Vector4::new(v[0], v[1], v[2], v[3]),
				Vector4::new(v[4], v[5], v[6], v[7]),
				Vector4::new(v[8], v[9], v[10], v[11]),
			]
		}
	}
}

impl std::ops::Index<usize> for Matrix4x3 {
	type Output = Vector4;
	fn index(&self, i: usize) -> &Self::Output {
		&self.v[i]
	}
}

impl std::ops::IndexMut<usize> for Matrix4x3 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.v[i]
	}
}

impl fmt::Debug for Matrix4x3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// write!(
		// 	f,
		// 	"Matrix4x3 [\n    {}, {}, {}, {},\n    {}, {}, {}, {},\n    {}, {}, {}, {},\n]",
		// 	self[0][0], self[0][1], self[0][2], self[0][3],
		// 	self[1][0], self[1][1], self[1][2], self[1][3],
		// 	self[2][0], self[2][1], self[2][2], self[2][3],
		// )
		writeln!(f, "Matrix4x3")?;
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
