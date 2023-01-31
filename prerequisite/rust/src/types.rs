
/*
Primitive Types--
Integers: u8, i8, u16 , i16, u32, u64, i64, u128, i128 (number of bits in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/



pub fn run () {
  //Default is i32 
  let x = 1 ;
  
  //Default is "f64"
  let y = 2.5 ;


  //Add explicit type 

  let z: i64 = 1234567893456 ;
  
  //Find max size 
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  //Boolean

  let is_active = true ;

  println!("{:?}", is_active) ;


  let a1 = 'a';

  //unicode characters

  let face  = '\u{1F600}' ;

  println!("{:?}", (a1, face))
  


}