fn main() {
    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of a vector
    println!("The lenghth of Vec::new is: {}",v.len());

    // Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    // printing the size of the vector
    println!("\n The length of the macro is: {}",v.len());
}
