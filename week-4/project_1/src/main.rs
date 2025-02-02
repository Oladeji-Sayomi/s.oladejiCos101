use std::io;

fn main() {
println!("Enter value of a");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Not a valid string");
let mut a:f64 = input1.trim().parse().expect("Not a valid number");

println!("Enter value of b");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("Not a valid string");
let mut b:f64 = input2.trim().parse().expect("Not a valid number");

println!("Enter value of c");
let mut input3 = String::new();
io::stdin().read_line(&mut input3).expect("Not a valid string");
let mut c:f64 = input3.trim().parse().expect("Not a valid number");

let d:f64 = (b * b) - (4.0 * (a * c));
let mut x1:f64 = (-b + d.sqrt()) / (2.0 * a) ;

println!("The first root of the quadratic equation (ax² + bx + c) is: {}",x1);

let mut x2:f64 = (-b - d.sqrt()) / (2.0 *a);

println!("The second root of the quadratic equation (ax² + bx + c) is: {}",x2);

if d == 0.0  {
    println!("There is only one real root of the quadratic equation.");
}
else if d > 0.0 {
    println!("There are two distinct roots of the quadratic equation.");
}
else if d < 0.0 {
    println!("There are no real roots of the quadratic equation.");
}
}


