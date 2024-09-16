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



// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// /* Fix function signature */
// fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
// { foo.b }

// fn main()
// {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }

//9. Methods

// /* Make it work by adding proper lifetime annotations */
// struct ImportantExcerpt{
//     part: &'static str,
// }

// impl ImportantExcerpt {
//     fn level<'a>(&self) -> i32 {
//         3
//     }
// }

// fn main() {}


//10. elision

/* Remove all the lifetimes that can be elided */

// fn input(x: &i32) {
//     println!("`annotated_input`: {}", x);
// }

// fn pass(x: &i32) -> &i32 { x }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// struct Owner(i32);

// impl Owner {
//     // Annotate lifetimes as in a standalone function.
//     fn add_one(&mut self) { self.0 += 1; }
//     fn print(&self) {
//         println!("`print`: {}", self.0);
//     }
// }

// struct Person<'a> {
//     age: u8,
//     name: &'a str,
// }

// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {}

// &'STATIC AND T:'STATIC


/* Fill in the blank in two ways */
// fn main() {
//     let v: &'static str ="hello";
//     need_static(v);

//     println!("Success!")
// }

// fn need_static(r : &'static str) {
//     assert_eq!(r, "hello");
// }

//2.

// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut config: Option<&mut Config> = None;

// /* Make it work without changing the function signatures of `init`*/
// fn init() -> Option<&'static mut Config> {
//     Some(&mut Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     })
// }


// fn main() {
//     unsafe {
//         config = init();

//         println!("{:?}",config)
//     }
// }

//3.


// fn main() {
//     let static_string:&'static str = "I'm in read-only memory";
//     {
//         // Make a `string` literal and print it:
//         println!("static_string: {}", static_string);

//         // When `static_string` goes out of scope, the reference
//         // can no longer be used, but the data remains in the binary.
//     }

//     println!("static_string reference remains alive: {}", static_string);
// }

//4.

// // Make a constant with `'static` lifetime.
// static NUM: i32 = 18;

// // Returns a reference to `NUM` where its `'static`
// // lifetime is coerced to that of the input argument.
// fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
//     &NUM
// }

// fn main() {
//     {
//         // Make an integer to use for `coerce_static`:
//         let lifetime_num = 9;

//         // Coerce `NUM` to lifetime of `lifetime_num`:
//         let coerced_static = coerce_static(&lifetime_num);

//         println!("coerced_static: {}", coerced_static);
//     }

//     println!("NUM: {} stays accessible!", NUM);
// }

//5.

/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}