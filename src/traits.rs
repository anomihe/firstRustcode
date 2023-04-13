// #! [deny(clippy::all)]
// //#[derive(Debug)]
// use std::fmt::{self, format};
// //traits 
// //traits are mixin in dart 
// //traits are shared functionality 


// struct Person{
//    first_name:String, 
//    last_name:String, 
//    age:u8
// }

// //define your own trait 

// trait HasFullName {
//     fn full_name(&self) ->String;
// }
// //implementing trait on Person
// impl HasFullName for Person{
//    fn full_name(&self) ->String {
//        format!("{} {}", self.first_name, self.last_name)
//    }
// }

// //trait with "new" function
// trait CanInitialeWithFullName {
//     fn new(full_name:&str)->Self;
// }

// impl CanInitialeWithFullName for Person{
//     fn new(full_name:&str)->Self {
//         let parts:Vec<&str> = full_name.split(' ').collect();
//         Person { first_name: parts[0].to_string(), last_name: parts[1].to_string(), age: 0 }
//     }
// }

// //another trait is the display trait which comes from fmt::display module 
// impl fmt::Display for Person{
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{} {} is {} years old",self.first_name, self.last_name, self.age)
//    }
// }

// //trait as parameters 
// //this take 
// fn print_full_name_and_age(value: &impl HasFullName){
//    println!("{}", value.full_name());
// }

// //trait bound signature
// fn print_details<T: HasFullName>(value:&T){
//    println!("{}", value.full_name()); 
// }
// //creating another trait to allow me test conforming to multiple traits 
// trait CanRun{
//    fn run(&self); 
// }

// impl CanRun for Person{
//    fn run(&self) {
//       println!("hello"); 
//    }
// }
// //conforming to multtiple traits 
// fn print_conformed<T: HasFullName + CanRun>(value:&T){
//    println!("{}", value.full_name()); 
//    value.run();
// }
// //we can also use where to make conforming neater 
// fn print_conformed_using_where<T>(value:&T) where T:HasFullName + CanRun{
//    println!("{}", value.full_name()); 
//    value.run();
// }

// //trait bound results cannot mix multiple types in mentioned 

// //traits can be implemented on other trait which is called binding 
// trait HasName{
//     fn first_name(&self) -> &str;
//     fn last_name(&self) -> &str;
// }

// trait HasFullName2 where Self:HasName {
//    //  fn full_name(&self) -> String{
//    //    format!("{} {}", self.first_name(), self.last_name())
//    //  }
//    //another way 
//    fn full_name(&self) ->String;
// }
// //implementation 
// impl<T> HasFullName2 for T where T:HasName {
//     fn full_name(&self) ->String {
//         format!("{} {}", self.first_name(), self.last_name())
//     }
// }

// //implementation hasname for person 

// impl HasName for Person{
//     fn first_name(&self) -> &str {
//         &self.first_name
//     }

//     fn last_name(&self) -> &str {
//         &self.last_name
//     }
// }

// fn main(){
// //    let person =Person{
// //     first_name: "John".to_string(),
// //     last_name: "Doe".to_string(),
// //     age: 30,
// // };
// // println!("{:?}",person)
// let person = Person::new("Anomihe Nmesoma");
// //println!("first is {}, last name is {} and age {}", person.first_name, person.last_name, person.age);
// //because of the fmt display we can the person directly to the module 
// //println!("{}", person);
// //print_full_name_and_age(&person);
//  let full_name = person.full_name();
//  println!("{}", full_name)
// }