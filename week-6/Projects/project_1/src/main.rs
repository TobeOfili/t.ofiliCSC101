use std::io ; 

fn calculate_trapezium_area(height:f64, base1:f64, base2:f64) {
   let trapezium_area = height/2.0*(base1 + base2);
   println!("The area of the trapezium is = {}",trapezium_area );
}

fn calculate_rhombus_area(d1:f64,d2:f64) {
   let rhombus_area = 0.5 * d1 * d2;
   println!("The area of the rhombus is = {}",rhombus_area );
}

fn calculate_parallelogram_area(base:f64, altitude:f64) {
   let parallelogram_area = base * altitude;
   println!("The area of the parallelogram is = {}",parallelogram_area );
}

fn calculate_cube_area(length:f64) {
   let cube_area = 6.0 * (length.powf(2.0));
   println!("The area of the cube is = {}",cube_area );
}

fn calculate_cylinder_volume(radius:f64, height:f64) {
   let cylinder_volume = (22.0/7.0) * radius.powf(2.0) * height ; 
   println!("The volume of the cylinder is = {}",cylinder_volume );
}



fn main() {
     let mut input1 = String::new();
   println!("What would you like to find : 
   1.) Area of a trapezium  
   2.) Area of a Rhombus 
   3.) Area of a Parallelogram
   4.) Area of a Cube
   5.) Volume of a Cylinder
    ");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let decision:u8 = input1.trim().parse().expect("Invalid Input")  ;
  

  if decision == 1 {
   let mut input2 = String::new();
   println!("Enter the trapezium height: ");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let height:f64 = input2.trim().parse().expect("Invalid Input")  ;

    let mut input3 = String::new();
   println!("Enter the first base of the trapezium: ");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let b1:f64 = input3.trim().parse().expect("Invalid Input")  ;

   let mut input4 = String::new();
   println!("Enter the second base of the trapezium: ");
   io::stdin().read_line(&mut input4).expect("Failed to read input");
   let b2:f64 = input4.trim().parse().expect("Invalid Input")  ;
  
   calculate_trapezium_area(height,b1,b2);
   }

  if decision == 2 {
    let mut input5 = String::new();
   println!("Enter the first diagonal of the rhombus: ");
   io::stdin().read_line(&mut input5).expect("Failed to read input");
   let diagonal1:f64 = input5.trim().parse().expect("Invalid Input")  ;

    let mut input6 = String::new();
   println!("Enter the second diagonal of the rhombus: ");
   io::stdin().read_line(&mut input6).expect("Failed to read input");
   let diagonal2:f64 = input6.trim().parse().expect("Invalid Input")  ;
  
   calculate_rhombus_area(diagonal1,diagonal2);
  }

  if decision == 3 {
      let mut input7 = String::new();
   println!("Enter the base of the parallelogram: ");
   io::stdin().read_line(&mut input7).expect("Failed to read input");
   let base:f64 = input7.trim().parse().expect("Invalid Input")  ;

    let mut input8 = String::new();
   println!("Enter the altitude of the parallelogram: ");
   io::stdin().read_line(&mut input8).expect("Failed to read input");
   let altitude:f64 = input8.trim().parse().expect("Invalid Input")  ;
  
   calculate_parallelogram_area(base,altitude);

  }

  if decision == 4 {
  let mut input9 = String::new();
   println!("Enter the length of the cube ");
   io::stdin().read_line(&mut input9).expect("Failed to read input");
   let length:f64 = input9.trim().parse().expect("Invalid Input")  ;

   calculate_cube_area(length);

  }

  if decision == 5 {

  let mut input10 = String::new();
   println!("Enter the radius of the cylinder: ");
   io::stdin().read_line(&mut input10).expect("Failed to read input");
   let radius:f64 = input10.trim().parse().expect("Invalid Input")  ;

    let mut input11 = String::new();
   println!("Enter the height of the cylinder ");
   io::stdin().read_line(&mut input11).expect("Failed to read input");
   let height:f64 = input11.trim().parse().expect("Invalid Input")  ;
  
   calculate_cylinder_volume(radius,height);
  }
 }
