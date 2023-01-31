// Arrays - fixed list where elements are the same data types 

pub fn run () {

  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  println!("{:?}", numbers);

  // Wont compile data types should be the same 
  // let array: [i32; 5] = ["this", 1 ,2 ,3] ;


  //indexing 

  println!("first value {}", numbers[0]) ;

  //Get array length 

  println!("The legnth of the array is {}", numbers.len());

  //Arrays are stack allocated 

  println!("Array occupies {} bytes ", std::mem::size_of_val(&numbers)) ;

  //Get Slice 

  let slice: &[i32] = &numbers ;
  println!("Slice: {:?}", slice);

  //Loop & mutate values 

  for x in numbers.iter_mut() {
    *x *= 2 ;
  }

  println! ("Numbers Vec: {:?}", numbers) ;

}