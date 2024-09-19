// fn main() {
//     // Increment via closures and functions.
//     fn function(i: i32) -> i32 { i + 1 }

//     // Closures are anonymous, here we are binding them to references
//     // 
//     // These nameless functions are assigned to appropriately named variables.
//     let closure_annotated = |i: i32| -> i32 { i + 1 };
//     let closure_inferred  = |i     |          i + 1  ;

//     let i = 1;
//     // Call the function and closures.
//     println!("function: {}", function(i));
//     println!("closure_annotated: {}", closure_annotated(i));
//     println!("closure_inferred: {}", closure_inferred(i));

//     // A closure taking no arguments which returns an `i32`.
//     // The return type is inferred.
//     let one = || 1;
//     println!("closure returning one: {}", one());

// }


//1. Capturing 

// /* Make it work with least amount of changes*/
// fn main() {
//     let color:String = String::from("green");

//     let print = || println!("`color`: {}", color);

//     print();
//     print();

//     // `color` can be borrowed immutably again, because the closure only holds
//     // an immutable reference to `color`. 
//     let _reborrow = &color;

//     println!("{}",color);
// }

//2.

/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
// fn main() {
//     let mut count:i32 = 0;

//     let mut inc = move || {
//         count += 1;
//         println!("`count`: {}", count);
//     };

//     inc();


//     let _reborrow: &i32 = &count; 

//     inc();

//     // The closure no longer needs to borrow `&mut count`. Therefore, it is
//     // possible to reborrow without an error
//     let _count_reborrowed: &i32 = &mut count; 

//    assert_eq!(count, 0);
// }

//3. 

/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
// fn main() {
//     let movable = Box::new(3);

//     let consume = || {
//         println!("`movable`: {:?}", movable);
//         take(&movable);
//     };

//     consume();
//     consume();
// }

// fn take<T>(_v: &T) {}


//4.

// fn main() {
//     let example_closure = |x:String| -> String { x };

//     let s: String = example_closure(String::from("hello"));

//     /* Make it work, only change the following line */
//     let n = example_closure(5.to_string());
// }

//5. Fn, FnMut, FnOnce

/* Make it work by changing the trait bound, in two ways*/
// fn fn_once<F>(abc: F)
// where
//     F: Fn(usize) -> bool,
// {
//     println!("{}", abc(3));
//     println!("{}", abc(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z|{z == x.len()})
// }


//6.

// fn main() {
//     let mut s: String = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}",s);
// }

// /* Fill in the blank */
// fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
//     f("hello")
// }


//7.

/* Fill in the blank */

// // A function which takes a closure as an argument and calls it.
// // <F> denotes that F is a "Generic type parameter"
// fn apply<F>(f: F) where
//     // The closure takes no input and returns nothing.
//     F: FnOnce() {

//     f();
// }

// // A function which takes a closure and returns an `i32`.
// fn apply_to_3<F>(f: F) -> i32 where
//     // The closure takes an `i32` and returns an `i32`.
//     F: Fn(i32) -> i32 {

//     f(3)
// }

// fn main() {
//     use std::mem;

//     let greeting = "hello";
//     // A non-copy type.
//     // `to_owned` creates owned data from borrowed one
//     let mut farewell = "goodbye".to_owned();

//     // Capture 2 variables: `greeting` by reference and
//     // `farewell` by value.
//     let diary = || {
//         // `greeting` is by reference: requires `Fn`.
//         println!("I said {}.", greeting);

//         // Mutation forces `farewell` to be captured by
//         // mutable reference. Now requires `FnMut`.
//         farewell.push_str("!!!");
//         println!("Then I screamed {}.", farewell);
//         println!("Now I can sleep. zzzzz");

//         // Manually calling drop forces `farewell` to
//         // be captured by value. Now requires `FnOnce`.
//         mem::drop(farewell);
//     };

//     // Call the function which applies the closure.
//     apply(diary);

//     // `double` satisfies `apply_to_3`'s trait bound
//     let double = |x| 2 * x;

//     println!("3 doubled: {}", apply_to_3(double));
// }

//8.

// /* Fill in the blank */
// fn main() {
//     let mut s = String::new();

//     let update_string = |str| -> String {s.push_str(str); s };

//     exec(update_string);
// }

// fn exec<'a, F: FnOnce(&'a str)-> String>( f: F) {
//     f("hello");
// }

//9. Input functions


// /* Implement `call_me` to make it work */
// fn call_me<F: Fn()> (f : F){
//     f();
// }

// fn function() {
//     println!("I'm a function!");
// }

// fn main() {
//     let closure = || println!("I'm a closure!");

//     call_me(closure);
//     call_me(function);
// }

//10,

// /* Fill in the blank using two approaches,
//  and fix the error */
// //  fn create_fn() -> impl Fn(i32) -> i32{
//  fn create_fn() -> Box<dyn Fn(i32) -> i32>{
//     let num:i32 = 5;

//     // How does the following closure capture the environment variable `num`
//     // &T, &mut T, T ?
//     // move |x| x + num
//     Box::new(move |x| x + num)
// }


// fn main() {
//     let fn_plain = create_fn();
//     fn_plain(1);
// }

//11

/* Fill in the blank and fix the error*/
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32>{

    let num: i32 = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
} 

fn main(){}