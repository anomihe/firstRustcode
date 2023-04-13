// #![deny(clippy::all)]
// #[derive(Debug)]

// //structure are classes 
// // struct Person{
// //     name:String, 
// //     age:u8, 

// // }


// // fn create_person(name:String, age:u8){
// //     let person = Person{
// //         name, 
// //         age
// //     };
// // }
// //tuple struct
// struct Point(f64,f64,f64); 
// //implementation for structs
//  impl Point {
//     //this function is mutating current
//     fn make_twice(&mut self){
//         self.0 *=2.0; 
//         self.1 *= 2.0; 
//         self.2 *= 2.0; 
//     }
//     //this function is returning the point while changing the value 
//     fn twice(&self)->Point{
//         Point(self.0*2.0, self.1*2.0, self.2*2.0)
//     }
//     fn describe(&self) {
//         println!("Point is at ({}, {}, {}) ", self.0,self.1, self.2)
//     }
//     fn zero() ->Point{
//         Point(0.0, 0.0, 0.0)
//     }
//  }
// //you can create multiple instance of struct 
//  //enum

 
// fn main(){
//     // let person = Person{
//     //     name:"John".to_string(), 
//     //     age:30,
//     // };
//     // struct update syntax 
//     // let person2 = Person{
//     //     name:"peter".to_string(), 
//     //     age:person.age
//     // }; 
//     // let person3 = Person{
//     //     name:"peter".to_string(), 
//     //     ..person
//     // }; 
// //calling tuples 
// //let point = Point(0.0,0.0,0.0);
// //println!("x={},y={}, z={}  ", point.0, point.1, point.2);
// //let p = Point(1.0, 2.0,3.0);
// //p.describe();
// //println!("{:?}", p);
//     // println!("{} is {} years old", person.name, person.age);
//     // println!("{} is {} years old", person2.name, person2.age);
//     // println!("{} is {} years old", person3.name, person3.age);
//  //create mutable and immutable Point instances 
//  let zero_point = Point::zero();
//  let zero_point2 = Point::zero();
//  let zero_point3 = Point::zero();
//  let mut point =Point(1.0,2.0,3.0); 
//  let twice =point.twice(); 
//  point.make_twice();
// }