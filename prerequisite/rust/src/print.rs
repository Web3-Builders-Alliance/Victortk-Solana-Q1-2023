pub fn run() {
  //Print to console

  println!("Hello, from the print.rs file");

  println!("Number: {}", 1) ;

  println!("{} is from  {} ", "Victor", "Norton");

  // positional arguments 

   println!("{0} is from {1} and {0} likes to {2}", "Victor", "Norton" , "Code" ) ;

  //  named arguments 

  println!("{name} likes to play {activity}", name = "Jonh", activity = "Baseball") ;

  //placeholder traits 

  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10 );

  // placeholder for debug trait 
  println!("{:?}",  (12, true, "hello")) ;

  //Base math 
  println!("10 + 10 = {}", 10 + 10); 

}