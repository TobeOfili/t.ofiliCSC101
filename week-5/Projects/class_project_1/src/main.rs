use std::io;

fn main() {
   
let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

   println!("\nEnter your value of a.");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a:f32 = input1.trim().parse().expect("Not a valid string");

   println!("\nEnter your value of b.");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b:f32 = input2.trim().parse().expect("Not a valid string");

   println!("\nEnter your value of c.");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let c:f32 = input3.trim().parse().expect("Not a valid string");

let discriminant:f32 = (b * b) - (4.0*a*c);

 if discriminant > 0.0 {
  let root1a:f32  = ((discriminant.sqrt())/2.0 ) + b;
  let root1b:f32 =  ((discriminant.sqrt())/2.0 ) - b ;

  println!("Your roots are {} and {}",root1b,root1a);

 }
else if discriminant == 0.0 {
 let root2:f32  = ((discriminant.sqrt())/2.0 ) + b;
 println!("\nThe roots of your equation are {} twice",root2);

 }

else if discriminant < 0.0 {
  println!("\nThere are no real roots for your inputs");


 }
else {
  println!("Invalid Input");
}
}
