// #! [deny(clippy::all)]


// //error handling


// // fn get_user_name()->Result<String, ()>{
// //     Ok("Ok john".to_string())
// //    //Err(())
// // } 


// fn get_first_name()->Result<String, String>{
//     //Ok("john".to_string())
//    Err("so there was no name ".to_string())
// }      



// fn get_last_name()->Result<String, String>{
//     //if an error is introduced in one the function
//     //it affects the other cause the error meesage to be seen on the console 
//    //Ok("favy john".to_string())
//   Err("there was no last name given ".to_string())
// } 


// fn get_full_name()->Result<String, String>{
//    //otionanl grabbing function 
//    let first_name = get_first_name()?;
//    let last_name =get_last_name()?; 
//    Ok(format!("{} {}", first_name, last_name))
//    //Err(())
// } 



// fn main() ->Result<(), Box<dyn std::error::Error>> {
// // let value :Result<&str, Box<dyn std::error::Error>> = Ok("Hello World");
// // match value{
// //     Ok(value)=>println!("{}", value), 
// //     Err(error)=>println!("{}", error), 
// // }

// //you can have result which is a void result value or errors 

// // let value :Result<&str, ()> = Ok("Hello World");
// //specifying error as a unit type
// // let value :Result<&str, ()> = Err(());
// // match value{
// //     Ok(value)=>println!("{}", value), 
// //     Err(_)=>println!("some error occured", ), 

// //expecting a value from result from a call site using expect

// // let value :Result<&str, ()> = Err(());
// // let unwrapped =value.expect("I was expecting your name");
// // let username = get_user_name().expect("Failed to fetch your result on the database");
// // println!("hello, {}!", username);
// //let user_name =
//  //get_user_name().expect_err("Failed to get username"); 
// //println!("hello user {}", user_name);

// //check Ok and eff with if-statement 


// //Early exist from errors
//  let full_name = get_full_name()?; 
// //  match full_name{
// //     Ok(name)=>println!("Hello {}!", name), 
// //     Err(_)=> println!("Error")
// //  }

// //Map ok in Result 
// // let length = full_name.map(|s| s.len()).unwrap_or_default();
// // println!("{}", length);

// // //one can also map error 
// // let error_length =full_name.map_err(|err| err.len()); 
// // println!("{:?}", error_length);


// //i can the change the signature of the main function and can return result
// Ok(())
// }