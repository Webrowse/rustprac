
// // FILL in the blanks and FIX errors
// // 1. Don't use `to_string()`
// // 2. Don't add/remove any code line
// fn main() {
//     let mut s: String = String::from("hello, ");
//     s.push_str("world");
//     s.push('!');

//     move_ownership(s.clone());

//     assert_eq!(s, "hello, world!");

//     println!("Success!");
// }

// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }


//2.

// // FILL in the blanks
// fn main() {  
//     let mut s = String::from("hello, world");
 
//     let slice1: &str = s.as_str(); // In two ways
//     assert_eq!(slice1, "hello, world");
 
//     let slice2 = &s[..5];
//     assert_eq!(slice2, "hello");
 
//     let slice3: &mut String = &mut s; 
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
 
//     println!("Success!");
//  }

//3.


// Question: how many heap allocations are happening here?
// Your answer: 2
fn main() {  
    // Create a String type based on `&str`
    // The type of string literals is `&str`
   let s: String = String::from("hello, world!");

   // Create a slice point to String `s`
   let slice: &str = &s;

   // Create a String type based on the recently created slice
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!githubtry");
}

