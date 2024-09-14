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

// /* Fix the error in three ways  */
// fn invalid_output<'a>() -> &'a str { 
//     "foo" 
// }

// fn main() {
//     let x = invalid_output();
//     println!("{}", x);
//     let y = invalid_output();
//     println!("{}", y);
//     println!("{}", x); 
// }


// // `print_refs` takes two references to `i32` which have different
// // lifetimes `'a` and `'b`. These two lifetimes must both be at
// // least as long as the function `print_refs`.
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// /* Make it work */
// // A function which takes no arguments, but has a lifetime parameter `'a`.
// fn failed_borrow() {
//     let _x = 12;

//     // ERROR: `_x` does not live long enough
//     let y: &i32 = &_x;
//     // Attempting to use the lifetime `'a` as an explicit type annotation 
//     // inside the function will fail because the lifetime of `&_x` is shorter
//     // than `'a` . A short lifetime cannot be coerced into a longer one.
// }

// fn main() {
//     let (four, nine) = (4, 9);
    
//     // Borrows (`&`) of both variables are passed into the function.
//     print_refs(&four, &nine);
//     // Any input which is borrowed must outlive the borrower. 
//     // In other words, the lifetime of `four` and `nine` must 
//     // be longer than that of `print_refs`.
    
//     failed_borrow();
//     // `failed_borrow` contains no references to force `'a` to be 
//     // longer than the lifetime of the function, but `'a` is longer.
//     // Because the lifetime is never constrained, it defaults to `'static`.
// }



//6.

// /* Make it work by adding proper lifetime annotation */

// // A type `Borrowed` which houses a reference to an
// // `i32`. The reference to `i32` must outlive `Borrowed`.
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// // Similarly, both references here must outlive this structure.
// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// // An enum which is either an `i32` or a reference to one.
// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number    = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }

//7.

// /* Make it work */

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// fn main()
// { 
//   /* 'a tied to fn-main stackframe */
//   let var_a = 35;
//   let example: Example;
  
//   {
//     /* Lifetime 'b tied to new stackframe/scope */ 
//     let var_b = NoCopyType {};
    
//     /* fixme */
//     example = Example { a: &var_a, b: &var_b };
//     println!("(Success!) {:?}", example);
//   }
  
// }


//8.



#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* Fix function signature */
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
{ foo.b }

fn main()
{
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}