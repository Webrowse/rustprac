#[allow(unused_variables)]
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
 
    //2.

    let x = 5;
    let _y = &x;

    // Modify this line only
    assert_eq!(5, x);

    println!("Success!2");

//3.
let s = String::from("hello, ");

borrow_object(&s);

println!("Success!3");

//4.
let mut s1 = String::from("hello, ");

push_str( &mut s1);

println!("Success!4");

//5.
let mut s = String::from("hello, ");

// Fill the blank to make it work
let p = &mut s;

p.push_str("world");

println!("Success!5");
}
//3.
fn borrow_object(_s: &String) {}
//4.
fn push_str(s1: &mut String) {
    s1.push_str("world")
}

