// #! [deny(clippy::all)]
// use std::ops::AddAssign;
// //#[derive(Debug)]

// // struct Point<T>{
// //     x:T, 
// //     y:T, 
// // }
// // //implement a 
// // impl<T> Point<T>{
// //      fn move_offset(&mut self, x:T, y:T) where T:AddAssign {
// //         self.x += x; 
// //         self.y +=y;
// //      } 
// // }

// // //implementing addsign on point itself 
// // //what the next line of code means that we are implementing
// // //on Point as long the data that add sign is carrying 
// // //conforms to addsign 
// // impl <T :AddAssign> AddAssign for Point<T>{
// //     fn add_assign(&mut self, other: Self) {
// //         self.x += other.x; 
// //         self.y += other.y;
// //     }
// // }

// // //implememting partialEq
// // //this allows us to implement equality on struct 
// // //even if they are generic 
// // impl<T:PartialEq> PartialEq for Point<T> {
// //    fn eq(&self, other: &Self) -> bool {
// //        self.x == other.x && self.y == other.y
// //    } 
// // }

// //more examples on traits and generics 
//  trait CanRun{
//     fn run(&self);
//  }
//  trait CanWalk{
//     fn walk(&self);
//  }

// //as long as the vector can run then the entire vector can run 
// impl<T:CanRun> CanRun for Vec<T>{
//     fn run(&self) {
//         for item in self{
//             item.run();   
//         }
//     }
// }
// impl<T:CanWalk> CanWalk for Vec<T>{
//     fn walk(&self) {
//         for item in self{
//             item.walk();   
//         }
//     }
// }


// struct Person{
//     name: String,
// }
// impl CanWalk for Person{
//     fn walk(&self) {
//         println!("{} is walking", self.name); 
//     }
// }
// impl CanRun for Person{
//     fn run(&self) {
//         println!("{} is walking", self.name); 
//     }
// }

// struct Elephant{
//     name:String,
// }  

// impl CanWalk for Elephant{
//     fn walk(&self) {
//         println!("{} is walking", self.name);
//     }
// }
// fn main(){
// // let p1= Point{x:2, y:7};
// // let p2 = Point{x:4.0, y:7.4};
// // //Point is generic struct we can also create a sring out of it
// // let p3 = Point{x:"nmesoma", y:"ken"};
// // let mut p = Point{x:5, y:5}; 
// // p.move_offset(5, 3);
// // println!("{:?}", p);

// // let  mut p1 = Point{x:1.0, y:2.0}; 
// // let p2 = Point{x:3.0, y:4.0};
// // // p1 +=p2; 
// // // println!("{:?}",p1);
// // //the code below is working because partialeq was implemented 
// // if p1 == p2{
// //     println!("they are equal");
// // }else{
// //     println!("thy are not equal");
// //}

// let people = vec![
//     Person{name:"John".to_string(),},
//     Person{name:"Jane".to_string(),},
//     Person{name:"Joe".to_string(),}

// ];
// people.run(); 
// people.walk(); 
// let elephant = vec![
//     Elephant{name:"John".to_string(),},
//     Elephant{name:"Jane".to_string(),},
//     Elephant{name:"Joe".to_string(),}

// ];
// //elephant.run(); 
// elephant.walk();

// }