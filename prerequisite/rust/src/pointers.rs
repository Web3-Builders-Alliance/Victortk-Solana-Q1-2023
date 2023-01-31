



pub fn run () {

  // Primitive Array

  let arr1 = [1,2,3];
  let arr2 = arr1 ;

  println!("Vars: {:?}", (arr1, arr2));
  //With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

  //Vector with error moved value 
  // let vec1 = [1,2,3];
  // let vec2 = vec1 ;

  // println!("Values: {:?}", (vec1,vec2));


  //vectors with reference
  let vec1 = [1,2,3];
  let vec2 = &vec1 ;

  println!("Values: {:?}", (vec1,vec2));

   

}