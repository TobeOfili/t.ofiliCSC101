use std::io;

fn main() {
   
let mut poundo_total:i32 = 0 ; 
let mut fried_rice_total:i32 = 0 ; 
let mut amala_total:i32 = 0 ; 
let mut eba_total:i32 = 0 ; 
let mut white_rice_total:i32 = 0 ; 

loop {

    println!(" \n            Menu                            Price
 P = Poundo Yam/Edinkaiko Soup              - N3,200
 F = Fried Rice & Chicken                   - N3,000
 A = Amala & Ewedu Soup                     - N2,500
 E = Eba & Egusi Soup                       - N2,000
 W = White Rice & Stew                      - N2,500
");

let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();
let mut input4 = String::new();
let mut input5 = String::new();
let mut input6 = String::new();
let mut input7 = String::new();

// first input 
println!("\nWhat would you like to eat?");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let decision = input1;

let poundo = "P" ;
let fried_rice = "F" ;
let amala = "A" ;
let eba = "E" ;
let white_rice = "W" ;

// decision case for poundo 
if decision.trim() == poundo  {
    println!("What quantity do you want ?");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let poundo_quantity:i32 = input2.trim().parse().expect("Not a valid string");

let poundo_amount:i32 = poundo_quantity * 3200 ;
 println!("That will be N{}",poundo_amount );
poundo_total = poundo_amount 
}

// decision case for fried rice
else if decision.trim() == fried_rice  {
    println!("What quantity do you want ?");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let fried_rice_quantity:i32 = input3.trim().parse().expect("Not a valid string");

let fried_rice_amount:i32 = fried_rice_quantity * 3000 ;
 println!("That will be N{}",fried_rice_amount );
fried_rice_total = fried_rice_amount
}

// decision case for amala
else if decision.trim() == amala   {
    println!("What quantity do you want ?");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let amala_quantity:i32 = input4.trim().parse().expect("Not a valid string");

let amala_amount:i32 = amala_quantity * 2500 ;
 println!("That will be N{}",amala_amount );
amala_total = amala_amount
}

// decision case for eba
 else if decision.trim() == eba {
    println!("What quantity do you want ?");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let eba_quantity:i32 = input5.trim().parse().expect("Not a valid string");

let eba_amount:i32 = eba_quantity * 2000 ;
 println!("That will be N{}",eba_amount );
eba_total = eba_amount
}

// decision case for white rice
else if decision.trim() == white_rice  {
    println!("What quantity do you want ?");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let white_rice_quantity:i32 = input6.trim().parse().expect("Not a valid string");

let white_rice_amount:i32 = white_rice_quantity * 2500 ;
 println!("That will be N{}",white_rice_amount );
white_rice_total = white_rice_amount
}

//loop case decision
println!("Do you want anything else ?  ");
io::stdin().read_line(&mut input7).expect("Failed to read input");
let decision2 = input7; 

//condition to end the program
if decision2.trim() == "no" {

let  total = poundo_total + fried_rice_total + amala_total + eba_total + white_rice_total ;
 if total < 10000 {
    println!("\n\nThank you for buying from us\n Your total is N{}", total);
break ;}
 
 // condition for 5% discount 
 else if total > 10000 {
    let conditional_total = (total * 95 )/100;
    println!("\n\nThank you for buying from us\n[You were given a 5% discount because your total purchase was above N10000 !!] \n\n Your total is N{}", conditional_total);
break ;}


}
//condition to loop the program
else if decision2.trim() == "yes" { continue ;}
}
}
