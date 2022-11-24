fn main() {
  //declaring the variable principal
 let principal:f64 = 210000.00;
  //declaring the variable depreciation_rate
 let depreciation_rate:f64 = 5.00;
  //declaring the variable time
 let time:f64 = 3.00;
  //declaring the variable amount
 let amount:f64 = principal * (1.00 - depreciation_rate/100.00).powf(time);
 
 
 println!("The value of the tv after {} years is N{}", time,amount);
 
 }

    
    
    
    
    
