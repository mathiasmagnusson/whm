use crate::{Float, Vector4};

use std::{fmt, ops};

#[derive(Clone, PartialEq)]
pub struct Matrix4x4([Vector4; 4]);

impl Matrix4x4 {
	pub const fn zero() -> Self {
		Self([Vector4::new(0.0, 0.0, 0.0, 0.0); 4])
	}
	pub const fn identity() -> Self {
		Self([
			Vector4::new(1.0, 0.0, 0.0, 0.0),
			Vector4::new(0.0, 1.0, 0.0, 0.0),
			Vector4::new(0.0, 0.0, 1.0, 0.0),
			Vector4::new(0.0, 0.0, 0.0, 1.0),
		])
	}
	pub const fn width() -> usize {
		Vector4::len()
	}
	pub const fn height() -> usize {
		4
	}
}

impl From<[Vector4; 4]> for Matrix4x4 {
	fn from(v: [Vector4; 4]) -> Self {
		Self(v)
	}
}

impl From<[[Float; 4]; 4]> for Matrix4x4 {
	fn from(v: [[Float; 4]; 4]) -> Self {
		Self([
			Vector4::from(v[0]),
			Vector4::from(v[1]),
			Vector4::from(v[2]),
			Vector4::from(v[3]),
		])
	}
}

impl From<[Float; 4*4]> for Matrix4x4 {
	fn from(v: [Float; 4*4]) -> Self {
		Self([
			Vector4::new(v[0], v[1], v[2], v[3]),
			Vector4::new(v[4], v[5], v[6], v[7]),
			Vector4::new(v[8], v[9], v[10], v[11]),
			Vector4::new(v[12], v[13], v[14], v[15]),
		])
	}
}

impl ops::Index<usize> for Matrix4x4 {
	type Output = Vector4;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl ops::IndexMut<usize> for Matrix4x4 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl ops::Neg for Matrix4x4 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		[
			-self[0],
			-self[1],
			-self[2],
			-self[3],
		].into()
	}
}

impl ops::Add for Matrix4x4 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		[
			self[0] + rhs[0],
			self[1] + rhs[1],
			self[2] + rhs[2],
			self[3] + rhs[3],
		].into()
	}
}

impl ops::AddAssign for Matrix4x4 {
	fn add_assign(&mut self, rhs: Self) {
		self[0] += rhs[0];
		self[1] += rhs[1];
		self[2] += rhs[2];
		self[3] += rhs[3];
	}
}

impl ops::Sub for Matrix4x4 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		[
			self[0] - rhs[0],
			self[1] - rhs[1],
			self[2] - rhs[2],
			self[3] - rhs[3],
		].into()
	}
}

impl ops::SubAssign for Matrix4x4 {
	fn sub_assign(&mut self, rhs: Self) {
		self[0] -= rhs[0];
		self[1] -= rhs[1];
		self[2] -= rhs[2];
		self[3] -= rhs[3];
	}
}
