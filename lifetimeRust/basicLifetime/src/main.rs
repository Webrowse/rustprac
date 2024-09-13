// /* Make it work by adding proper lifetime annotation */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let x = "Hello, world!";
//     let y = "Goodbye, world!";
//     println!("{}", longest(x, y));
// }


// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn invalid_output<'a>() -> &'a str { 
    "foo" 
}

fn main() {
    let x = invalid_output();
    println!("{}", x);
    let y = invalid_output();
    println!("{}", y);
    println!("{}", x); 
}