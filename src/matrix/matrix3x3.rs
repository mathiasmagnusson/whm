use crate::{Float, Vector3};

use super::{Matrix, Matrix2x2};

use std::{fmt, ops};

#[derive(Clone, PartialEq)]
pub struct Matrix3x3([Vector3; 3]);

impl Matrix3x3 {
}

impl Matrix for Matrix3x3 {
	fn zero() -> Self {
		Self([Vector3::new(0.0, 0.0, 0.0); 3])
	}
	fn identity() -> Self {
		Self([
			Vector3::new(1.0, 0.0, 0.0),
			Vector3::new(0.0, 1.0, 0.0),
			Vector3::new(0.0, 0.0, 1.0),
		])
	}
	fn width() -> usize {
		Vector3::len()
	}
	fn height() -> usize {
		3
	}
	fn det(&self) -> Float {
		self[0][0] * Matrix2x2::from([
			self[1][1], self[1][2],
			self[2][1], self[2][2],
		]).det()
		- self[0][1] * Matrix2x2::from([
			self[1][0], self[1][2],
			self[2][0], self[2][2],
		]).det()
		+ self[0][2] * Matrix2x2::from([
			self[1][0], self[1][1],
			self[2][0], self[2][1],
		]).det()
	}
}

impl From<[Vector3; 3]> for Matrix3x3 {
	fn from(v: [Vector3; 3]) -> Self {
		Self(v)
	}
}

impl From<[[Float; 3]; 3]> for Matrix3x3 {
	fn from(v: [[Float; 3]; 3]) -> Self {
		Self([
			Vector3::from(v[0]),
			Vector3::from(v[1]),
			Vector3::from(v[2]),
		])
	}
}

impl From<[Float; 3*3]> for Matrix3x3 {
	fn from(v: [Float; 3*3]) -> Self {
		Self([
			Vector3::new(v[0], v[1], v[2]),
			Vector3::new(v[3], v[4], v[5]),
			Vector3::new(v[6], v[7], v[8]),
		])
	}
}

impl ops::Index<usize> for Matrix3x3 {
	type Output = Vector3;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl ops::IndexMut<usize> for Matrix3x3 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl fmt::Debug for Matrix3x3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "┌╴        ╶┐")?;
		writeln!(f, "│{: >2} {: >2} {: >2} │", self[0][0], self[0][1], self[0][2])?;
		writeln!(f, "│          │")?;
		writeln!(f, "│{: >2} {: >2} {: >2} │", self[1][0], self[1][1], self[1][2])?;
		writeln!(f, "│          │")?;
		writeln!(f, "│{: >2} {: >2} {: >2} │", self[2][0], self[2][1], self[2][2])?;
		writeln!(f, "└╴        ╶┘")?;

		Ok(())
	}
}

impl ops::Neg for Matrix3x3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		[
			-self[0],
			-self[1],
			-self[2],
		].into()
	}
}

impl ops::Add for Matrix3x3 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		[
			self[0] + rhs[0],
			self[1] + rhs[1],
			self[2] + rhs[2],
		].into()
	}
}

impl ops::AddAssign for Matrix3x3 {
	fn add_assign(&mut self, rhs: Self) {
		self[0] += rhs[0];
		self[1] += rhs[1];
		self[2] += rhs[2];
	}
}

impl ops::Sub for Matrix3x3 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		[
			self[0] - rhs[0],
			self[1] - rhs[1],
			self[2] - rhs[2],
		].into()
	}
}

impl ops::SubAssign for Matrix3x3 {
	fn sub_assign(&mut self, rhs: Self) {
		self[0] -= rhs[0];
		self[1] -= rhs[1];
		self[2] -= rhs[2];
	}
}

impl ops::Mul<Float> for Matrix3x3 {
	type Output = Matrix3x3;
	fn mul(self, s: Float) -> Self::Output {
		[
			s * self[0],
			s * self[1],
			s * self[2],
		].into()
	}
}

impl ops::Mul for Matrix3x3 {
	type Output = Matrix3x3;
	fn mul(self, rhs: Self) -> Self::Output {
		fn calc_element(a: &Matrix3x3, b: &Matrix3x3, r: usize, c: usize) -> Float {
			(0..Matrix3x3::width()).map(|i| a[r][i]*b[i][c]).sum()
		}
		[
			calc_element(&self, &rhs, 0, 0), calc_element(&self, &rhs, 0, 1), calc_element(&self, &rhs, 0, 2),
			calc_element(&self, &rhs, 1, 0), calc_element(&self, &rhs, 1, 1), calc_element(&self, &rhs, 1, 2),
			calc_element(&self, &rhs, 2, 0), calc_element(&self, &rhs, 2, 1), calc_element(&self, &rhs, 2, 2),
		].into()
	}
}
