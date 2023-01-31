use std::env ;

pub fn run () {

  let args: Vec<String> = std::env::args().collect() ; 
  let name = "Victor" ;
  let command  = args[1].clone() ;
 
  // println!("Args {:?}", args);

  println!("Command: {}", command) ;

  if command  == "hello" {
    println!("Hi {}, how are you?", name);
  }else if command == "victor"{
    println!("Hi {}, how are you?", name);

  }else {
    println!("try again")
  }
  

}