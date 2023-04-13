// #![deny(clippy::all)]

// fn main(){
// let message = say_hello_world();
// say_hello_world_two();
// with_argument("nmesoma");
// let name=say_name("peace");

// //inline function 

// let say_hello_to=|name:&str| format!("hello,  {}",name);
// //an inline function that accepts multiple argument 
// let full_name = |first_name:&str, Last_name:&str|format!("mr {} {}", first_name, Last_name);
// //inline function with no bracket 
//  let multiply_by_2 = |x:i32|x*2;

// //  let ask_for_age = ||{
// //     let mut age =String::new();
// //     println!("how old are you?"); 
// //     std::io::stdin().read_line(buf:&mut age).expect()
// //  }
// //pointers within a function 
// let multiply_by_2 = |x:i32| x*2;
// let ptr = multiply_by_2; 
// let result = ptr(10);
// println!("result {}", result);

// println!("{}", message);
// println!("hello {}", name);
// println!("welcome {}", say_hello_to("nmeso")) ;
// println!("{} ", full_name("anomihe", "Nmesoma"));
// println!("{} ", multiply_by_2(2));
// }

// //functions 
// //a function that return something 
// fn say_hello_world()->String{
//      String::from("Hello, world!")
// }

// //function that does not return anything 
// //->() this a unit type
// fn say_hello_world_two(){
//     let message = String::from("Hello World version two"); 
//     println!("{}", message);
// }
//  //passing an argument to a function 
//  fn with_argument(messga:&str){
//     println!("hello {}", messga);
//  }

//  // function with a  return 
//  fn say_name(to_person:&str) -> String{
// format!("Hello, {}!", to_person)
//  }

//  //passing a function as an argument 
// fn process_name(name:&str,callback:fn(&str)-> ()){}