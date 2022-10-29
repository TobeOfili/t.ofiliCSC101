fn main() {
 //declaring the variable toshiba_price
 let toshiba_price:f64 = 450000.00 * 2.00;
 //declaring the variable mac_price 
 let mac_price:f64 = 1500000.00 * 1.00;
 //declaring the variable hp_price    
 let hp_price:f64 = 750000.00 * 3.00;
 //declaring the variable dell_price      
 let dell_price:f64 = 2850000.00 * 3.00;   
  //declaring the variable acer_price
 let acer_price:f64 = 250000.00 * 1.00;
 
  //declaring the variable total_sales and setting the formula 
 let total_sales:f64 = toshiba_price + mac_price + hp_price + dell_price + acer_price;
 //declaring the variable sales_average and setting the formula
 let sales_average:f64 = total_sales / 5.00;

 println!("The total sales are {} and the average is {}",total_sales,sales_average);
 
}
    
    
    
    
    
