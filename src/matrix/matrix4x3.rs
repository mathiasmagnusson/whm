use crate::Vector3;
use crate::Vector4;

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
	pub fn solve(mut self) -> Vector3 {
		self[0] /= self[0][0];

		self[1] -= self[0] * self[1][0];

		self[2] -= self[0] * self[2][0];

		self[1] /= self[1][1];

		self[2] -= self[1] * self[2][1];

		self[2] /= self[2][2];

		self[1] -= self[2] * self[1][2];

		self[0] -= self[2] * self[0][2];

		self[0] -= self[1] * self[0][1];

		Vector3::new(self[0][3], self[1][3], self[2][3])
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
