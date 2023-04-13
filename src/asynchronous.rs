// #! [deny(clippy::all)]
// //use std::async_iter;

// use tokio::time::{sleep,Duration};


// //pcmp this means package create module and path 
// //package is a set of create 
// //


// //asynchronous rust 

// //async function 

// use futures::executor::block_on;
// //using future traits 
// use futures::Future;
// // async fn get_name() -> String{
// //     "John".to_string()
// // }

// //changing our function sinature to return a future using traits 
//  fn call_api_one() -> impl Future<Output = String>{
//  async{
//   sleep(Duration::from_secs(1)).await; 
//   "Kenneth".to_string()
//  }
// }

// fn call_api_two() -> impl Future<Output = String> {
//  async{
//   sleep(Duration::from_secs(1)).await;
//   "nmesoma".to_string()
//  }
  
// }

// //use tokio  await package for future 
// // async fn call_api_one() -> String{
// //    sleep(Duration::from_secs(4)).await;
// //    "kenneth".to_string()
// // }

// // async fn call_api_two() -> String{
// //   sleep(Duration::from_secs(1)).await;
// //   "nmesoma".to_string()
// // }


// //lifetime of variables returned by futures 
// //they need to live as long as the future itself lives 
// // to be very sure that pur variable is living as long as our future we to ensure that our work 
// //is done withing the asnyc block 

// //asynchronous functions can move varibles from out a block into the block


// fn get_async_name() -> impl Future<Output = String> {
//  let name = "John Doe".to_string(); 
//  //th name variable is outside the async block so be ablle to call it insdie the async 
//  //block 
//  //to ensure that variable lives on as long as the async function we will mark the async function as move

//  async move{ 
// format!("hello {}, welcome",  name)
//  }
//  }

// //using tokio package we have to turn our fution into async function 
// #[tokio::main]
// async fn main(){
//   //awaiting on a future 
//   //   let namr =block_on(get_name()); 
//   // println!("hello, {}!", namr);
//   let one = call_api_one().await; 
//   println!("hello {}", one);
//   let two = call_api_two().await; 
//   println!("hello {}", two);
// } 