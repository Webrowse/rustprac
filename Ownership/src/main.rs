
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

    let x2: Box<i32> = Box::new(5);
    
    let mut y2:Box<i32> = Box::new(1);      
    
    *y2 = 4;
    
    assert_eq!(*x2, 5);

    println!("Success!7");
//8.
let t3:(String, String) = (String::from("hello"), String::from("world8"));

   let _s3: String = t3.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t3.1);

   //9.
   let t4 = (String::from("hello"), String::from("world9"));

   // Fill the blanks
   let (s1c, s2c) = t4.clone();

   println!("{:?}, {:?}, {:?}", s1c, s2c, t4); // -> "hello", "world", ("hello", "world")

}