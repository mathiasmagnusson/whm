#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod generic;
mod vector;
mod matrix;

pub use generic::*;
pub use vector::*;
pub use matrix::*;

#[cfg(test)]
mod tests;
