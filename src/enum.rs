// #![deny(clippy::all)]


// //enum is used to create same type 

// struct Size {
//     width:f32, 
//     height:f32
// }

// //implementation of enums with unname associated values 

// impl Shapes{
//     fn area(&self) ->f32{
//         match self{
//             Shapes::Rectangle(_x, _y,size )=>size.width *size.height,
//             Shapes::Circle(_x, _y, radius)=>3.14*radius*radius,
//         }
//     }
// }


// //match is an expression 
// enum Pet{
//     Cat{name:String}, 
//     Dog{name:String}, 
// }

// // enum AnimalTYpe{
// //     Dog, 
// //     Cat, 
// //     Rabbit, 
// // }

// // enum Shapes{
// //     Circle{radius:f64, center:(f64, f64)}, 
// //     Rectangle{width:f64, height:f64},
// // }
// enum Shapes{
//     Circle(f32, f32, f32), 
//     Rectangle(f32,  f32, Size),
// }


// fn main(){
// //creating an instance of enumeration
// // let fluffy = AnimalTYpe::Dog;
// // //match is same a switch case in other languages 
// // match fluffy {
// //     AnimalTYpe::Cat=> println!("Meow!"), 
// //     // AnimalTYpe::Dog => println!("Woof!"), 
// //     // AnimalTYpe::Rabbit => println!("Hoor!"),
// //     //line 21 is the default case 
// //     _ => println!("some other animal"),
// // }

// // let rectangle  = Shapes::Rectangle { width: 3.0, height: 4.0 ,};
// //  if let Shapes::Rectangle { width, height }= rectangle{
// //     println!("width ={}, height = {}", width, height)
// //  }
// //  //using match 
// //  match  rectangle {
// //      Shapes::Rectangle { width, height }=>{
// //         println!("{}", width *height);
// //      }
// //      _ =>println!("not a rectangle"), 
// //  }

// // let rectangle = Shapes::Rectangle(1.0, 1.0, Size { width: 4.0, height: 5.0 } );
// // //matching with if statemment 
// //   if let Shapes::Rectangle(x,y , Size { width, height })=rectangle{
// //     println!("{} {} {} {}", x,y,width, height);
// //   }
// //   //using match 
// //   match rectangle{Shapes::Rectangle(x, y, Size { width , height })=>{
// //     println!("Rectangle: x:{}, y:{}, width:{}, height: {}", x,y,width, height)
// //   }
// //   _ =>println!("not one of us "),}

// //   let area = rectangle.area(); 
// //   println!("Area is {}", area);

// let mut pet = Pet::Cat { name: "fluffy".to_string() };
// let name = match pet {  
//     Pet::Cat { name }=>name, 
//     Pet::Dog { name }=>name, 
// };
// println!("hello {}", name);
// }
