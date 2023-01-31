//Primitive str = str = Imutable fixed-legnth 
//String = Growable , heap allocated data structure - use when you need to modify or own string data 

pub fn run () {
 
  let mut  hello = String::from("Hello ");

  print!("{}", hello);

  println!("Length: {}" , hello.len());

  //push a char

  hello.push('w');

  //push a string 

  hello.push_str("orld!");

  //capcity in bytes 
  println!("Capacity: {}", hello.capacity());

  //Contains 

  println!("Contains 'World' {}", hello.contains("World"));

  //Replace 
  println!("Replace: {}", hello.replace("world", " There"));

  //Loop through string by whitespace 
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  //Create string with capcity 

  let mut s = String::with_capacity(10);

  s.push('a');
  s.push('b');

  //Assertion testing 
  assert_eq!(2,s.len());
  assert_eq!(10,s.capacity());


  println!("{}", hello);


}