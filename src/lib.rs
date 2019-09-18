#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod vector3;
mod vector4;

pub use vector3::Vector3;
pub use vector4::Vector4;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
