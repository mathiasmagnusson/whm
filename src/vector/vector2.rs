use crate::Float;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2([Float; 2]);

impl Vector2 {
	pub const fn new(v0: Float, v1: Float) -> Self {
		Self([v0, v1])
	}
	pub fn x(&self) -> Float {
		self[0]
	}
	pub fn y(&self) -> Float {
		self[1]
	}
	pub fn len() -> usize {
		2
	}
	pub fn mag_sq(&self) -> Float {
		self[0] * self[0] +
		self[1] * self[1]
	}
	pub fn mag(&self) -> Float {
		self.mag_sq().sqrt()
	}
	pub fn normalized(&self) -> Self {
		let mag = self.mag();
		Self([
			self[0] / mag,
			self[1] / mag,
		])
	}
	pub fn normalize(&mut self) {
		let mag = self.mag();
		self[0] /= mag;
		self[1] /= mag;
	}
}

impl From<(Float, Float)> for Vector2 {
	fn from(v: (Float, Float)) -> Self {
		Self ([
			v.0,
			v.1,
		])
	}
}

impl From<[Float; 2]> for Vector2 {
	fn from(v: [Float; 2]) -> Self {
		Self(v)
	}
}

impl std::ops::Index<usize> for Vector2 {
	type Output = Float;
	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl std::ops::IndexMut<usize> for Vector2 {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl std::ops::Neg for Vector2 {
	type Output = Vector2;
	fn neg(self) -> Self::Output {
		[-self[0], -self[1]].into()
	}
}

impl std::ops::Add for Vector2 {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self ([
			self[0] + other[0],
			self[1] + other[1],
		])
	}
}

impl std::ops::Sub for Vector2 {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self ([
			self[0] - other[0],
			self[1] - other[1],
		])
	}
}

impl std::ops::Mul<Float> for Vector2 {
	type Output = Self;
	fn mul(self, scalar: Float) -> Self::Output {
		Self ([
			self[0] * scalar,
			self[1] * scalar,
		])
	}
}

impl std::ops::Div<Float> for Vector2 {
	type Output = Self;
	fn div(self, scalar: Float) -> Self::Output {
		Self ([
			self[0] / scalar,
			self[1] / scalar,
		])
	}
}

impl std::ops::AddAssign for Vector2 {
	fn add_assign(&mut self, other: Self) {
		self[0] += other[0];
		self[1] += other[1];
	}
}

impl std::ops::SubAssign for Vector2 {
	fn sub_assign(&mut self, other: Self) {
		self[0] -= other[0];
		self[1] -= other[1];
	}
}

impl std::ops::MulAssign<Float> for Vector2 {
	fn mul_assign(&mut self, scalar: Float) {
		self[0] *= scalar;
		self[1] *= scalar;
	}
}

impl std::ops::DivAssign<Float> for Vector2 {
	fn div_assign(&mut self, scalar: Float) {
		self[0] /= scalar;
		self[1] /= scalar;
	}
}
