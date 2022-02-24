use std::io;


pub fn fibo(n: u128) -> u128 {
	if n < 2 {
		n
	} else {
		fibo(n - 1) + fibo(n - 2) }
}

pub fn get_n() -> u128 {
	let mut input: String = String::new();

	io::stdin()
		.read_line(&mut input)
		.expect("Couldn't read input.");

	let input: u128 = input.trim().parse().expect("That's not a number you idiot!");

	input
}

// noinspection SpellCheckingInspection
pub fn get_fibo_array(n: u128) -> Vec<u128> {
	let mut array: Vec<u128> = Vec::new();

	for i in 1..(n + 1) {
		array.push(fibo(i))
	}

	array
}

