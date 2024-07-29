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

//borrow_object(&s);

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
//6.
let c = '中';

let r1 = &c;
// Fill the blank，dont change other code
let ref  r2 = c;

assert_eq!(*r1, *r2);

// Check the equality of the two address strings
assert_eq!(get_addr(r1),get_addr(r2));

println!("Success!6");
//7.
let s3 = String::from("hello");

    let r1 = &s3;
    let r2 = &s3;

    println!("{}, {}", r1, r2);

    println!("Success!7");
//8.
     // Fix error by modifying this line
     let mut s2 = String::from("hello, ");

     borrow_object(&mut s2);
 
     println!("Success!8");
//9.
     let mut s = String::from("hello, ");

    borrow_object1(&s);
    
    s.push_str("world");

    println!("Success!9");
}
//3.
//fn borrow_object(_s: &String) {}
//4.
fn push_str(s1: &mut String) {
    s1.push_str("world")
}
//6.
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
//8.
fn borrow_object(s2: &mut String) {}
//9.
fn borrow_object1(adarsh: &String) {}