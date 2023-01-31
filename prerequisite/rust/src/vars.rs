//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language 


pub fn run () {

  let name = "Victor";
  let mut age = 25 ;

   println!("I am aged {} ", age) ;
  println!("My name is {}", name) ;

  age = 26 ;

  println!("Next year my gae will be {}" , age ) ;

  // Define constant 
  const ID: i32 = 001 ;
  println!("ID: {}", ID) ;

  //Assign multiple vars 

  let (my_name, my_age) = ("Victor", 25) ;

  println!("Multiple vars {} : {} ", my_name, my_age );

}