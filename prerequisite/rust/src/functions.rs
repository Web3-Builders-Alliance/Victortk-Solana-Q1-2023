pub fn run () {

   greeting("Hello", "Victor") ;


   //Closure 

   let add_nums = |n1: i32, n2: i32| n1 + n2 ;

   println!("C Sum {}", add_num(3,3));
}

fn greeting(greet: &str, name: &str){
   println!("{} {}",greet, name)
}