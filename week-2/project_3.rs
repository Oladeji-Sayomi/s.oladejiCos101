fn main() {
	let p = 210_000.0;
	let r = 5.0;
	let n = 1.0;

	//first year
	let a_1 = p * (1.0 - (r / 100.0)) * n;
	let d_1 = p - a_1;
	
	//second year
	let a_2 = a_1 * (1.0 - (r / 100.0)) * n;
	let d_2 = a_1 - a_2;

	//third year
	let a_3 = a_2 * (1.0 - (r / 100.0)) * n;
	let d_3 = a_2 - a_3;

	let a = a_3;
	let d = d_1 + d_2 + d_3;

	println!("The value of the TV after 3 years is {}", a);
	println!("The total depreciation of the TV after 3 years is {}", d);
	
}