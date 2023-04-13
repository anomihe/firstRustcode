// #![deny(clippy::all)]

// mod ownership;

// const THEIR_AGE: u8 = 34; 

// fn main() {
//     //creating a mutable variable in 
//     let name  = "John";
//     let last_name = "kenneth";  
//     let age : i32 = -40; 
//     //use underscore to separate digit just like commas 
//     //use f for floating points 
//     let yeats:f32 = 65.0; 
  

//    //variable shadowing= is creating a variable with the name in the same scope 
  
//   let schol:&str = "good"; 
//   let scool:&str ="hekr"; 
//   //tuples
// //used to store datas that are unrelated 
// let scores:(i32, &str) = (34, "bio"); 
// let (first, sub) = scores;

// //ownership 
// let name1:String = String::from("john"); 
// let joke:&String = &name1; 

// //mutable reference 
// empty_string(&mut joke);

// //Dangling reference 
// let name: &String =get_name();

//    println!("he lllo {} {} {} {} {}", name  , last_name, yeats, schol, first);
// println!("hello {} {}", name1, joke);
 
// }

// // mutable refrence 
// fn empty_string(value: &mut String){
//     value.clear();
// }
// //dangling reference 
// fn get_name()-> &String{
// &"John".to_string()
// }