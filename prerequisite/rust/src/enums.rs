//Enums are types which have a few definitive values 
enum DayOfWeek {

  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday

}


impl DayOfWeek {

  fn select_activity(d: DayOfWeek) {
  //Perform action depending on day of the wek
    match d {
      DayOfWeek::Monday => println!("Running"),
      DayOfWeek::Tuesday => println!("Jogging"),
      DayOfWeek::Wednesday => println!("Walking"),
      DayOfWeek::Thursday => println!("Coding"),
      DayOfWeek::Friday => println!("Reading"),
      DayOfWeek::Saturday => println!("Singing"),
      DayOfWeek::Sunday => println!("Talking"),
    }
  
  }

}


pub fn run() {

   let day =  DayOfWeek::Saturday ;
   let day2 =  DayOfWeek::Monday ;
   let day3 =  DayOfWeek::Wednesday;
   let day4 =  DayOfWeek::Friday ;

   DayOfWeek::select_activity(day);
   DayOfWeek::select_activity(day3);
   DayOfWeek::select_activity(day2);
   DayOfWeek::select_activity(day4);
}