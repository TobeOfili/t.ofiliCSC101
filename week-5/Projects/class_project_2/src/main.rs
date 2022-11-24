use std::io;

fn main() {
    
   let mut input1 = String::new();
   let mut input2 = String::new();
  

   println!("\nIs the employee experienced.");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let employee_experience = input1;


if employee_experience.trim() == "yes" {
   println!("\nHow old is the employee.");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let employee_age:f32 = input2.trim().parse().expect("Not a valid string");


   if employee_age >= 40.0 {
   println!("\nThe employees incentive is N1,560,000");
}

  else if employee_age >= 30.0 && employee_age < 40.0 {
 println!("\nThe employees incentive is N1,480,000");
  }
 
 else if employee_age < 29.0 {
    println!("\nThe employees incentive is N1,300,000");
 }

else {
    println!("Invalid input");
}}

else if employee_experience.trim() == "no" {
println!("\nThe employees incentive is N100,000");

}

else {
    println!("Invalid input");
}
}
