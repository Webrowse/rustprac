
// fn main() {
//     let s1 = "hello";
//     /* Fill in the blank */
//     let s = format!("{}, world!", s1);
//     assert_eq!(s, "hello, world!");
// }


// fn main() {
//     /* Fill in the blanks to make it print:
//     Hello world, I am 
//     Sunface!
//     */
//     print!("hello world, ");
//     println!("I am");
//     print!("Sunface!");
//  }

//Debug traits 


// /* Fill in the blanks and Fix the errors */
// #[derive(Debug)]
// struct Structure(i32);

// fn main() {
//     // Types in std and Rust have implemented the fmt::Debug trait
//     println!("{} months in a year.", 12);

//     println!("Now {:?} will print!", Structure(3));
// }



// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8
// }

// fn main() {
//     let person = Person { name:  "Sunface".to_string(), age: 18 };

//     /* Make it output: 
//     Person {
//         name: "Sunface",
//         age: 18,
//     }
//     */
//     println!("{:#?}", person);
// }




use core::fmt;
struct Structure(i32);


struct Deep(Structure);

impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }

}
fn main() {    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}