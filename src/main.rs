mod factor;

pub(crate) fn main() {
	let numbers: [u32; 2] = [6, 28];
	for num in numbers {
		let result = factor::prime_factors(num);
		println!("{:?}", result);
	}

	let c: u32 = factor::greatest_common_divisor(numbers[0], numbers[1]);
    println!("Greater Common Divisor is: {c}");
	let d: u32 = factor::least_common_multiple(numbers[0], numbers[1]);
    println!("Least Common Multiple is: {d}");
}