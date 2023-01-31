//Loops used to iterate until a conditional is met


pub fn run () {

  let mut count  = 0 ; 

  //Infite Loop 
  loop {
    count += 1 ;
    println!("Number {}", count) ;

    if count == 20 {
      break;
    }
  }

  //while Loop (FizzBuzz)

  while count <= 100 {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    }else if count % 5 == 0 {
      println!("buzz")
    }else {
      println!("{}", count)
    }
  }

  for x in 0..100{

  }

  
}