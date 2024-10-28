fn main() {
	let qty_1 = 2.0;
	let qty_2 = 1.0;
	let qty_3 = 3.0;
	let qty_4 = 3.0;
	let qty_5 = 1.0;
	let t = 450_000.0;
	let m = 1_500_000.0;
	let h = 750_000.0;
	let d = 2_850_000.0;
	let a = 250_000.0;

	//sum
	let toshiba = t * qty_1;
	let mac = m * qty_2;
	let hp = h * qty_3;
	let dell = d * qty_4;
	let acer = a * qty_5;
	
	let sum = toshiba + mac + hp + dell + acer;

	//average
	let average = sum / (qty_1 + qty_2 + qty_3 + qty_4 + qty_5);

	println!("The sum of the sales in P.M. Okeke and Sons Ltd's sales record is {}", sum);
	println!("The average of the sales in P.M. Okeke and Sons Ltd's sales record is {}", average);

}
