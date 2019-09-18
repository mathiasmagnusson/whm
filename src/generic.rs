pub trait Integer {
	fn factorial(self) -> Self;
	fn permutations(n: Self, k: Self) -> Self;
	fn combinations(n: Self, k: Self) -> Self;
	fn binomial(a: Self, b: Self, n: Self) -> Self;
	fn power(self, exp: Self) -> Self;
}

macro_rules! for_all_integer_types {
	($int:ty) => {
		impl Integer for $int {
			#[allow(unused_comparisons)]
			fn factorial(self) -> Self {
				assert!(self >= 0);
				(1..=self).fold(1, |prod, v| prod * v)
			}
			fn permutations(n: Self, k: Self) -> Self {
				((n-k+1)..=n).fold(1, |prod, v| prod * v)
			}
			fn combinations(n: Self, k: Self) -> Self {
				Self::permutations(n, k) / k.factorial()
			}
			fn binomial(a: Self, b: Self, n: Self) -> Self {
				(0..=n).map(|k| Self::combinations(n, k) * a.power(n-k)*b.power(k)).sum()
			}
			fn power(self, mut exp: Self) -> Self {
				let mut base = self;
				let mut acc = 1;

				while exp > 1 {
					if (exp & 1) == 1 {
						acc = acc * base;
					}
					exp /= 2;
					base = base * base;
				}
				if exp == 1 {
					acc = acc * base;
				}

				acc
			}
		}
	};
}

for_all_integer_types!(u8);
for_all_integer_types!(u16);
for_all_integer_types!(u32);
for_all_integer_types!(u64);
for_all_integer_types!(u128);
for_all_integer_types!(usize);

for_all_integer_types!(i8);
for_all_integer_types!(i16);
for_all_integer_types!(i32);
for_all_integer_types!(i64);
for_all_integer_types!(i128);
for_all_integer_types!(isize);
