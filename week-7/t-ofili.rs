use std::io;

 fn pub_service() {
    let mut decision1 = String::new();
println!("Is the staff a public servant ?");
io::stdin().read_line(&mut decision1).expect("Failed to read input");  

if decision1.trim().to_lowercase()  == "yes" {
   let mut decision2 = String::new();
println!("How many years of experience does the member of staff have ? ");
io::stdin().read_line(&mut decision2).expect("Failed to read input");  
let decision2:i32 = decision2.trim().parse().expect("Input not an integer");

let mut decision3 = String::new();
println!("What proffesion does the staff member do
	1) Office Administrator
	2) Academic
	3) Lawyer
	4) Teacher
	");
io::stdin().read_line(&mut decision3).expect("Failed to read input");  
let decision3:i32 = decision3.trim().parse().expect("Input not an integer");

  if (decision2 < 3) && (decision3 == 1) {
println!("The member of staff is an Intern");
  }
2
  else if (decision2 < 3) && decision3 == 2 {
println!("The person isnt yet a staff member");
  }

   else if (decision2 < 3) && decision3 == 3 {
println!("The member of staff is a paralegal");
  }

 else if (decision2 < 3) && decision3 == 4 {
println!("The member of staff is a placement");
  }


 else  if (decision2 < 6) && (decision2 > 2) && (decision3 == 1) {
println!("The member of staff is an Administrator");
  }

  else if (decision2 < 6) && (decision2 > 2) && decision3 == 2 {
println!("The person is a Research Assistant");
  }

   else if (decision2 < 6) && (decision2 > 2) && decision3 == 3 {
println!("The member of staff is a junior associate");
  }

 else if (decision2 < 6) && (decision2 > 2) && decision3 == 4 {
println!("The member of staff is a classroom teacher");
  }



else  if (decision2 < 9) && (decision2 > 4) && (decision3 == 1) {
println!("The member of staff is a Senior Administrator");
  }

  else if (decision2 < 9) && (decision2 > 4) && decision3 == 2 {
println!("The person is a PHD candiate");
  }

   else if (decision2 < 9) && (decision2 > 4) && decision3 == 3 {
println!("The member of staff is an associate");
  }

 else if (decision2 < 9) && (decision2 > 4) && decision3 == 4 {
println!("The member of staff is a Snr teacher");
  }



else  if (decision2 < 11) && (decision2 > 8) && (decision3 == 1) {
println!("The member of staff is an office manager");
  }

  else if (decision2 < 11) && (decision2 > 8) && decision3 == 2 {
println!("The person is a Post doc Researcher");
  }

   else if (decision2 < 11) && (decision2 > 8) && decision3 == 3 {
println!("The member of staff is a senior associate 1-2");
  }

 else if (decision2 < 11) && (decision2 > 8) && decision3 == 4 {
println!("The member of staff is a leading teacher");
  }


 else  if (decision2 < 14) && (decision2 > 9) && (decision3 == 1) {
println!("The member of staff is a director ");
  }

  else if (decision2 < 14) && (decision2 > 9) && decision3 == 2 {
println!("The person is a senior lecturer");
  }

   else if (decision2 < 14) && (decision2 > 9) && decision3 == 3 {
println!("The member of staff is a senior associate 3-4");
  }

 else if (decision2 < 14) && (decision2 > 9) && decision3 == 4 {
println!("The member of staff is a deputy principal");
  }

  else  if decision2 >= 14 {
println!("The member of staff is a CEO");
  }

  else if decision2 >= 14 {
println!("The person is a Dean");
  }

   else if decision2 >= 14 {
println!("The member of staff is a Partner");
  }

 else if decision2 >= 14 {
println!("The member of staff is a principal");
  }
else {

	println!("Invalid Input");
}
}

else if decision1.trim().to_lowercase() == "no"{
	println!("This Checker doesnt apply to the member of staff");
}

else { println!("Invalid Input");}
   
}



fn geopo_merger() {
	let name_of_commisioner:[&str;5] = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu",
	"Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
  
    let ministry:[&str;5] = ["Internal Affairs","Justice",
	"Defense","Power and steel","Petroleum"];
  
    let geopolitical_zone:[&str;5] = ["South West","North East",
	"South South","South West","South East"];
  
  for index in 0..5 {
println!("\n{}.) The name of the commisioner is {}.\n    The commisioners ministry is {}.
	Their geopolitical zone is {}\n ",index + 1,name_of_commisioner[index],ministry[index],
	geopolitical_zone[index] );
}
}


fn main() {

let mut decision = String::new();
println!("Which division would you like to use :
1.) The Geopolitical zoning division
2.) The Public Service Checker	");
io::stdin().read_line(&mut decision).expect("Failed to read input");  
let decision:i32 = decision.trim().parse().expect("Input not an integer");

if decision == 1 {
   geopo_merger(); 
   }

else if decision == 2 {
   pub_service();
}

else { 
println!("Invalid Input");
}
}