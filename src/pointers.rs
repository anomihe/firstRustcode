// #! [deny(clippy::all)]
// use std::rc::Rc;
// use std::cell::{Cell, RefCell};
// //POINTERS
// //to understand pointer we need to understand stack and heap, 
// //stack operate LIFO which is last in first out, 
// //while heap is a random access 

// //first pointer in rust is Box data type 
// //Box stores in a heap 

// use std::ops::Deref;


// //to understand Box we need to create our own impleementation of Box 
// struct BoxedValue<T>{
//     value:T,
// }
// //implementing boxedvalue 
// impl<T> BoxedValue<T>{
//     //Boxed value can be returned or Self
//     fn new(value:T)->Self{
//         BoxedValue { value }
//     }
// }
// //implement the box dereferencing 
// impl<T> Deref for BoxedValue<T>  {
//     type Target =T;
//     fn deref(&self) ->&Self::Target{
//         &self.value
//     }
// }

// // //implicit Deref coercion in functions
// // //Rust helps call deref() for us automatically 
// // fn print_integer(value:&i32){
// //     println!("{}", value);
// // }

// //RC which means reference counting, which is a useful pointer 
// //this disallows mutation of the wrapped value 
// //you can create weak reference to Rc, this reference won't own the data 

// //creating a person struct for cell 
// //cell allows internal mutabilty
// //cell allows interior mutability 
// //it is unsafe 
// // struct Person{
// //     name:String, 
// //     age: Cell<u8>,
// // }
// // impl Person {
// //     fn increment_age(&self)->u8{
// //         self.age.set(self.age.get()+1);
// //         self.age.get()
// //     }
// // }


// // there is another pointer in rust called 
// //RefCell for multiple immutable borrows or 1 mutable 
// //this a safer version of using cell
// //the rules of RefCel are enforced at runtime unlike Box 
// //RefCell is only allowed in single threaded enviroments
//  //RefCell can be borrowed immutable or mutably 



 
// fn main(){

// //example of Box with dereferencing 
// //  let age = Box::new(22);
// //  let twice =*age *2;
// // println!("{}", twice);  

// //dereferencing a Boxed value 
// //let age = BoxedValue::new(22); 
// // let twice = *age *2 ; 
// // println!("{}", twice);


// //i can also derefernce a value 
// //deref points to the dereferenced value 
// // let actual_age =age.deref(); 
// // println!("{}", actual_age);

// // //i can also point to the underlyig dereferenced value 
// //  let underlying_value =*age;
// // println!("{}", underlying_value);
// //the *ptr is a shorthand for *(ptr.deref())


// //  let actual_age =*age; 
// //  let ref_to_value =age.deref(); 
// //  let other =*(age.deref());

// //  // pass it to the function using ampersand &
// //   let value =BoxedValue::new(10);
// //   print_integer(&value);

// // //using rc 
// // let array = vec!["john".to_string(), "jane".to_string()];
// // let rc =Rc::new(array); 
// // //getting a weak reference to the rc 
// // let weak = Rc::downgrade(&rc);
// // //the drop function allows us drop the rc valus 
//  //   drop(rc);
// // // let value = weak.upgrade().unwrap();
// // // println!("{:?}",value);
// // //because upgrade returns an optional value i can use match 
// // match weak.upgrade(){
// //     Some(rc) => println!("{:?}", rc),
// //     None => println!("none"),
// // }

// //we can also clone the Rc to create a new rc Object 
// // let rcs =rc.clone();
// // //droping rc to check our clone
// // drop(rc);
// // println!("{:?}", rcs);
// //another method of doing the cloning 
//  //let rc2 =Rc::clone(&rc);

// //  //Mutability of Rc, Rc doesn't allow this but std::cell::Cell does
// // let person = Person{
// //     name:"John".to_string(), 
// //     age:Cell::new(20),
// // };
// // let new_age = person.increment_age(); 
// // println!("{}", new_age);

// let ref_cell = RefCell::new(vec![1,2,4,3]); 
// let mut mut_ref  = ref_cell.borrow_mut();
// let len = ref_cell.borrow().len();
// mut_ref.push(100);
// println!("length is{}", len);

// //there are ways to combine pointers 

// }