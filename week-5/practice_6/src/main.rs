 use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();


println!("Enter your lower bound: ");
 io::stdin().read_line(&mut input1).expect("Failed to read input");
let lower_bound:i32 = input1.trim().parse().expect("Input not an integer");

println!("Enter your upper bound: ");
 io::stdin().read_line(&mut input2).expect("Failed to read input");
let upper_bound:i32 = input2.trim().parse().expect("Input not an integer");

for x in lower_bound..upper_bound{
println!("Count level is {}",x );

}



}


