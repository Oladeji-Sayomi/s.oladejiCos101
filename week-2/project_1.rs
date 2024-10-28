fn main() {
	let p = 520_000_000.0;
	let r = 10.0;
	let t = 1.0;

	//first year
	let a = p * (1.0 + (r / 100.0)) * t;
	let ci_1 = a - p;

	//second year
	let a_1 = a * (1.0 + (r / 100.0)) * t;
	let ci_2 = a_1 - a;

	//third year
	let a_2 = a_1 * (1.0 + (r / 100.0)) * t;
	let ci_3 = a_2 - a_1;

	//fourth year
	let a_3 = a_2 * (1.0 + (r / 100.0)) * t;
	let ci_4 = a_3 - a_2;

	//fifth year
	let a_4 = a_3 * (1.0 + (r / 100.0)) * t;
	let ci_5 = a_4 - a_3;

	//total compound interest
	let ci = ci_1 + ci_2 + ci_3 + ci_4 + ci_5;
	println!("The total compound interest is {}", ci);

}
