
// //optional data type 

// fn main(){
//     //we some function to create an option value tat will have 
//     //while none as on that won't have any value 
//     let value = Some(20); 
//     //let name:Option<&str> = Some("Nmesome");
//     let name:Option<&str> = None;
//     //use unwrap to unwrap the optional value 
//     //safely unwrap using match 
//     match name{
//        Some(name)=>println!("Hello, {}",name), 
//        None=>println!("There was no name")
//     }
//    //  //unsafely unwrapping the value option 
//    //   let unwrapped_name =  name.expect("no name");
//    //   println!("expected {}", unwrapped_name);
//      //forcely unwrapping  
//      let named = Some("peter"); 
//      let unwrapped_name =named.unwrap(); 
//      println!("name is {}", unwrapped_name);
   
//      //mutating option values
//      let mut age = Some(20); 
//      match age.as_mut(){
//        Some(age)=>*age +=10, 
//        None => println!("No age"),
//      }
//      println!("Age is {}", age.unwrap());
   
//      //unwrapping multiple optionals with tuples 
//      let age1:Option<i8> = Some(20);
//      let age2:Option<i8> = Some(30);
//      let age3:Option<i8> = Some(40);
//      if let(Some(age_1), Some(age_2), Some(age_3))= (age1, age2, age3) {
//        println!("{} {} {}", age_1, age_2, age_3);
//      }
   
//      //unwrap with default value 
//    //  let name1:Option<&str> = None; 
//      //how ever if the value was provided the default value will be replaced 
//      let name1:Option<&str> = Some("kennedy"); 
//      let unwrapped1 = name1.unwrap_or("hopeless"); 
//      println!("name is {}", unwrapped1); 
//      //unwrap with function 
//    //   let name2:Option<&str> = None; 
//    let name2:Option<&str> = Some("oma "); 
//      let unwrapped2 = name2.unwrap_or_else(||{
//        //do some work 
//        "Nmesoma "
//      });
//      println!("name is {}", unwrapped2);
   
//    //check if Option is "Some" or "None"
//    let name2:Option<&str> = None; 
//    //there is also is_none
//    if name2.is_some() {
//        println!("there was a value"); 
//    }else {
//        println!("there was no value");
//    }
   
//    //unwrapping a default value 
//     let my_age:Option<i32> = None; 
//     let my_age = my_age.unwrap_or_default(); 
//     println!("{}", my_age);
//     //Mapping an option 
//    //  let multipl_age:Option<i32> =None; 
//    //  let age_multiplied_by_2 = multipl_age.map(|multipl_age: Option<i32>| multipl_age*2);
//    //options can also be returned as a value 
//    //Optons and optional are used interchangeably but they mean the same thing
   
//    }