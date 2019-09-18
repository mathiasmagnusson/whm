#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

mod generic;
mod vector;

pub use generic::*;
pub use vector::*;

#[cfg(test)]
mod tests;
