// #! [deny(clippy::all)]


// //lifetime in rust 

// //sample of function that won't compile 
// //'static added to data type to tell the rust analyzer that 
// //the data will live for a live time 
// fn get_full_name()->&'static str {
//    "kennth anoms"
// }

// //generic lifetime annotations 
// //now when the return value of the is changedd the return 
// //there will be a life time error again because theerewas a mismatch in the life time specifier
// //so what can is to se the same generic specifier 
// fn get_randome_name<'l>(a:&'l str,b:&'l str )->&'l str{
//    b
// }

// //lifetime operators in structs 
// //solving the error missing lifetime annotations in structs
//  struct Person<'a>{
//    first_name:&'a str,
//    last_name:&'a str
//  }
//  //creating implementation of structs with lifetime 
//  impl<'a> Person<'a> {
//      //defining function person which has reference to self 
//      fn first_char_of_first_name(&self)-> &str{
//          &self.first_name[0..1]
//      }

//      //returning a full name(can't use &str)
//      fn get_full_name(&self)->String{
//          format!("{} {}", self.first_name, self.last_name)
//      }
//  }
// //lifetime are fundamental to how Rust manages memory
// //lifetime in enums
// enum Animal <'a>{
//    Dog{name:&'a str}, 

// }

//  //Lifetime elision
// fn get_first_name(full_name:&str) ->&str{
//    full_name
// }

// //Lifetime rules and there are 3 of them in rust 

// //Lifetime rule 1 
// //compiler assigns lifetime to every paramter that's a reference 

// //lifetime rule 2 
// //Single input lifetime is assigned to all outputs

// //Lifetime rule 3 
// //if &self or &mut self in parameters, lifetime of self is assigned to output 

// //Define a person struct with a lifetime specifier 

// fn main(){
// // let name = get_randome_name("ken", "joy"); 
// // println!("{}", name);
// } 