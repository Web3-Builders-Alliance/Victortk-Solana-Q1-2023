//Conditionals - Used to check the conditona of something

pub fn run () {

let age  = 18;

if age  >= 21 {
  println!("Higher")
}
else  {
  println!("Low")
}

let words = if age < 20-5 {
   "lower than"
}else if age < 35 && age > 20 {
  "In the middle"
}else {
  "very high"
};

println!("words")


}