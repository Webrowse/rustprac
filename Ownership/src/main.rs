
fn main() {
//1. Use as many approaches as you can to make it work
    let x = String::from("Hello world1");
    let y = x.clone();
    println!("{}, {}",x, y);

//2. Don't modify code in main!

    let s1 = String::from("Hello world2");
    let s2 = take_ownership(s1);

    println!("{}", s2);


// Only modify the code below!
fn take_ownership(s: String)->String {
    println!("{}", s);
    s
    
}
//3.
    let s = give_ownership();
    println!("{}", s);

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world3");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}
//4. Fix the error without removing any code

    let sa = String::from("Hello World4");

    print_str(sa.clone());

    println!("{}", sa);


fn print_str(sa: String)->String  {
    println!("{}",sa);
    sa
}
//5.
let x1:(i32, i32, (), &str) = (1, 2, (), "hello5");
let y1 = x1;
println!("{:?}, {:?}", x1, y1);

let sb: String = String::from("Hello ");
    
    let mut s1b: String = sb;

    s1b.push_str("World!");

    println!("Success!6");



}