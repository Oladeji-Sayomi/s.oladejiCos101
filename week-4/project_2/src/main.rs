use std::io;

fn main() {
    
    println!("Please enter if you are experienced or not.\nIf you are experienced, enter true.\nIf you are not, enter false");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:bool = input1.trim().parse().expect("Not a valid answer");

    println!("Please enter your age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experience == false
    {
        let incentive:i32 = 100_000;
    println!("Your annual incentive is: {}",incentive);
    }
    else if age >= 40   
    {
        let incentive:i32 = 1_560_000;
    println!("Your annual incentive is: {}",incentive);
    }
    else if age >= 30 && age < 40   
    {
        let incentive:i32 = 1_480_000;
    println!("Your annual incentive is: {}",incentive);
    }
    else if age < 28
    {
        let incentive:i32 = 1_300_000;
    println!("Your annual incentive is: {}",incentive);
    }
   }
