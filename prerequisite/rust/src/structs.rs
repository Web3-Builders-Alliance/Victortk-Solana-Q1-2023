 struct Color {

    red: u8 ,
    green:u8 ,
    blue: u8 ,
  }
  

//Tuple struct 
struct ColorTwo(u8,u8,u8) ;

struct Person {
  first_name:  String,
  last_name: String ,
}


impl Person {

  //Construct person 
  fn new(first: &str , last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  //Set last name

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
}

pub fn run () {

  let mut c = Color {
    red: 255,
    green: 0 ,
    blue: 0
  } ;

  println!("Color: r: {} g:{} b:{}", c.red ,c.green , c.blue ) ;

  let c = ColorTwo(0,0,255) ;

  println!("Color: r: {} g:{} b:{}", c.0 ,c.1 , c.2 ) ;


  let mut p  = Person::new("John", "Doe") ;
  println!("Person {} {}", p.first_name , p.last_name) ;

  

}