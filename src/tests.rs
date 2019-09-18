use super::*;
#[test]
fn factorial() {
	assert_eq!(0.factorial(), 1);
	assert_eq!(1.factorial(), 1);
	assert_eq!(2.factorial(), 2);
	assert_eq!(3.factorial(), 6);
	assert_eq!(4.factorial(), 24);
	assert_eq!(10.factorial(), 3628800);
}
#[test]
fn permutations() {
	assert_eq!(Integer::permutations(6, 3), 6 * 5 * 4);
	assert_eq!(Integer::permutations(6, 6), 6 * 5 * 4 * 3 * 2 * 1);
}
#[test]
fn combinations() {
	assert_eq!(Integer::combinations(6, 2), 15);
	assert_eq!(Integer::combinations(6, 6), 1);
	for n in 0..10 {
		assert_eq!(Integer::combinations(n, 0), 1);
		assert_eq!(Integer::combinations(n, 1), n);
		for k in 0..=n {
			assert_eq!(Integer::combinations(n, k), Integer::combinations(n, n - k));
		}
	}
}
#[test]
fn binomial() {
	assert_eq!(Integer::binomial(4, 5, 1), 4 + 5);
	assert_eq!(Integer::binomial(4, 5, 2), (4 + 5) * (4 + 5));
	assert_eq!(
		Integer::binomial(4, 5, 7),
		(4 + 5) * (4 + 5) * (4 + 5) * (4 + 5) * (4 + 5) * (4 + 5) * (4 + 5)
	);
}