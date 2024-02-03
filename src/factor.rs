pub fn greatest_common_divisor(a: u32, b: u32) -> u32 {
	if b != 0 {
		greatest_common_divisor(b, a % b)
	} else {
		a
	}
}

pub fn least_common_multiple(a: u32, b: u32) -> u32 {
	(a * b) / greatest_common_divisor(b, a % b)
}

pub fn prime_factors(mut n: u32) -> Vec<u32> {
	let mut factors = Vec::new();

	while n % 2 == 0 {
		factors.push(2);
		n /= 2;
	}

	for i in (3..=n).step_by(2) {
		while n % i == 0 {
			factors.push(i);
			n /= i;
		}
	}

	if n > 2 {
		factors.push(n);
	}

	factors
}