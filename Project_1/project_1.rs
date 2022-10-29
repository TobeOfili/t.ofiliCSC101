fn main() {
 //declaring the variable principal
 let principal:f64 = 520000000.00;
 //declaring the variable rate
 let rate:f64 = 10.00;
 //declaring the variable time
 let time:f64 = 5.00;
 //declaring the variable amount and setting the formula
 let amount:f64 = principal * (1.00 + rate/100.00).powf(time);
 //declaring the variable compound_interest and setting the formula
 let compound_interest:f64 = amount - principal;

 
 println!("The Compound Interest is N{}", compound_interest);
}
    
    
    
    
    
