use crate::Float;

mod matrix4x3;

pub use matrix4x3::*;

pub trait Matrix:
	std::ops::Index<usize, Output = Float>
	+ std::ops::IndexMut<usize>
	+ std::ops::Index<(usize, usize), Output = Float>
	+ std::ops::IndexMut<(usize, usize)>
{
	fn width(&self) -> usize;
	fn height(&self) -> usize;
	fn get(&self, col: usize, row: usize) -> &Float;
	fn get_mut(&mut self, col: usize, row: usize) -> &mut Float;
	// fn solve(this: Box<Self>) -> Vec<Float> {
	// 	for row in 1..this.height() {
	// 		for col in 0..row {
	// 			for x in 0..this.width() {

	// 			}
	// 		}
	// 	}
	// 	vec![]
	// }
}
