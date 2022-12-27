use std::io;

fn main() {
   let mut input1 = String::new();
   println!("How many siblings do you have  ");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let decision:i32 = input1.trim().parse().expect("Invalid Input")  ;
  
 let arr:[i32;decision] = [decision];

for val in arr.iter(){
   println!(" He has {} siblings",val );

}

}
