// //#[derive(Hash, Eq, PartialEq, Debug)]

// use std::collections::{HashMap, btree_map::Entry};

// //collection 
// //a function that returns a tuple 
// fn get_values()->(String, String, i32){
//     ("hello".to_string(), "world".to_string(), 30)
// }
// //tuples 
// //vector as it is called in rust but is generally known as list 


// fn main(){
//     let values = ("hello", "world", 30); 
//     //unpacking a tuple 
//     let hello = values.0; 
//     let world = values.1; 
//     let number = values.2; 
//     //unpacking tuples as a group 
//    let (ell, war, age) = values;
//    //ignor a parameter in atuple 
//    let (_, _, age) = values;
//   // println!("is values : {}", values.1);
// let (_,_hello, _) = get_values();
// println!("{}", get_values().2);
// //creating a fixed vector 
// let val =["nmeso", "ken"];  

// //getting a length of a vector 
// let length = val.len(); 
// //mapping vector(list)
// let numbw= [10,20]; 
// let doubled = numbw.iter().map(|x|x*2);
//  for vale in val.iter(){
//     println!("{}", vale);
//  } 
//  let mut unnumber = vec![1,3,5,6,7,9];
//  let mut unnumber2 = vec![1,3,5,6,7,9];
//  //add mut to make it a mutable vector 
//  //you can push and pop in vector using a push and pop function 
//  //if the vect is not a mutable vector push and pop function an not work 
// unnumber.push(11); 
// let last = unnumber.pop(); 
// //remove a value in a mutable vector using the clear functio
//  unnumber.clear(); 
//  //extending a vector 
//  unnumber.extend_from_slice(&[13,15,17]);
//  //moving a vector into anoto another vector 
// unnumber.append(&mut unnumber2);
// //using a contain function 
// if unnumber.contains(&3){
//     println!("yes");
// }else {
//     println!("no");
// }
// if unnumber2.is_empty(){
//     println!("yes")
// }else {
//     println!("no");
// }

// //HashMaps are dictionaries which are key value pairs 
// let mut hased = HashMap::new();
// hased.insert("nmesoma", "kenneth"); 
// //to check if a HashMap contains key
// if hased.contains_key("joy"){
//     println!("yes");
// }else{
//     println!("nope");
// }
// //removing a key 
// //hased.remove("nmesoma"); 
// //unsafely reading a value in map 
// let unsafe_read = hased["nmesoma"]; 
// //println!("unsafely read {}", unsafe_read);
// //safely reading a map using a get function 
// match   hased.get("nmesoma"){
//     Some(hase)=>println!("{}",hase ), 
//     None =>println!("not found"), 
// }
// //iterating over a hashmap 
//  for(&k, &v) in &hased{
//     println!("is iter {} {}", k,v)
//  }
// //retrieving a value in hasmap 
// let entry =hased.entry("nmesoma");
//  match entry{
//     std::collections::hash_map::Entry::Occupied( hash) => println!("found"),
//     //std::collections::hash_map::Entry::Vacant(_) => todo!(),
//     _=> println!("noy found"),
// }
// //inserting into HashMap if key absent 
// hased.entry("middel name").or_insert("chiebuka"); 

// //inserting custom structs into HashMap


// println!(" ne entery{:?}", hased);
// let mut nam = Per{name:"kenneth".to_string(), age:8};

// let mut value = HashMap::new(); 
// value.insert("name", nam.name); 
// value.insert("age", nam.age.to_string());

// //Iterators 
// let value_it = vec![1,2,3,5,6,7,8]; 
// //iterators can be a type 
// let iter =value_it.iter(); 
// let sum1:i32 =  iter.sum();
// println!("{}", sum1);
// //mapping an iterator 
// let multiply_by_2:Vec<i32> = value_it.iter().map(|v| v+1).collect(); 
// println!("{:?}",multiply_by_2);
// //owned iterator using into_iter goes through values themselves, owns the collection 
// let named = vec!["john", "mary", "candy", "nmesoma", "chiebuka"]; 
// // for name in  named.into_iter(){
// // println!("{}", name);
// // }
// //using filtering iterators 
// // for filterd_name in named.into_iter().filter(|name|name.len()==4){
// // println!("{}", filterd_name); 
// // }
// //iterating over maps
// //jumping using continue 
// // for name in named.into_iter(){
// //     if name.len() ==3{
// //         continue;
// //     }
// //     println!("is continued{}", name);
// // }
// //breaking out of an iteration using break 
// for name in named.into_iter(){
//     if name.len() ==3{
//         break;
//     }
//     println!("is continued{}", name);
// }
//  //println!("the length is {:?} and the mapped value {:?}", unnumber, unnumber2)
// }

// struct  Per{
//     name:String, 
//     age:u8,
// }